use crate::connection_string;
use crate::sqlite_path::ResolvedSqliteOptions;
use datasources::{DatasourceError, Result};
use rusqlite::Connection as SqliteDb;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

struct Live {
    in_use: u32,
    max_in_use: u32,
    overlap_at: Option<u32>,
}

pub(crate) struct SqlitePool {
    filename: String,
    options: ResolvedSqliteOptions,
    idle: Mutex<Vec<SqliteDb>>,
    opened: Mutex<u32>,
    closed: Mutex<bool>,
    live: Mutex<Live>,
    overlap: Condvar,
}

pub(crate) struct PooledConnection<'a> {
    pool: &'a SqlitePool,
    db: Option<SqliteDb>,
}

impl SqlitePool {
    pub(crate) fn new(filename: String, options: ResolvedSqliteOptions) -> Self {
        Self {
            filename,
            options,
            idle: Mutex::new(Vec::new()),
            opened: Mutex::new(0),
            closed: Mutex::new(false),
            live: Mutex::new(Live {
                in_use: 0,
                max_in_use: 0,
                overlap_at: None,
            }),
            overlap: Condvar::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn require_concurrent_checkouts(&self, n: u32) {
        self.live.lock().expect("sqlite").overlap_at = Some(n);
    }

    #[cfg(test)]
    pub(crate) fn max_in_use(&self) -> u32 {
        self.live.lock().expect("sqlite").max_in_use
    }

    fn acquired(&self) {
        let mut live = self.live.lock().expect("sqlite");
        live.in_use += 1;
        if live.in_use > live.max_in_use {
            live.max_in_use = live.in_use;
        }
        if let Some(need) = live.overlap_at {
            while live.in_use < need {
                live = self.overlap.wait(live).expect("sqlite");
            }
            self.overlap.notify_all();
        }
    }

    fn released(&self) {
        let mut live = self.live.lock().expect("sqlite");
        live.in_use = live.in_use.saturating_sub(1);
        self.overlap.notify_all();
    }

    pub(crate) fn close(&self) {
        *self.closed.lock().expect("sqlite") = true;
        self.idle.lock().expect("sqlite").clear();
    }

    pub(crate) fn checkout(&self) -> Result<Option<PooledConnection<'_>>> {
        if *self.closed.lock().expect("sqlite") {
            return Ok(None);
        }
        if let Some(db) = self.idle.lock().expect("sqlite").pop() {
            self.acquired();
            return Ok(Some(PooledConnection {
                pool: self,
                db: Some(db),
            }));
        }
        {
            let mut opened = self.opened.lock().expect("sqlite");
            if *opened >= self.options.pool_size {
                return Ok(None);
            }
            *opened += 1;
        }
        match open_pooled(&self.filename, &self.options) {
            Ok(db) => {
                self.acquired();
                Ok(Some(PooledConnection {
                    pool: self,
                    db: Some(db),
                }))
            }
            Err(error) => {
                *self.opened.lock().expect("sqlite") -= 1;
                Err(error)
            }
        }
    }

    fn checkin(&self, db: SqliteDb) {
        self.released();
        if *self.closed.lock().expect("sqlite") {
            return;
        }
        self.idle.lock().expect("sqlite").push(db);
    }
}

impl PooledConnection<'_> {
    pub(crate) fn db(&self) -> &SqliteDb {
        self.db.as_ref().expect("sqlite")
    }
}

impl Drop for PooledConnection<'_> {
    fn drop(&mut self) {
        if let Some(db) = self.db.take() {
            self.pool.checkin(db);
        }
    }
}

pub(crate) fn configure(db: &SqliteDb, options: &ResolvedSqliteOptions) -> Result<()> {
    if let Some(mode) = options.journal_mode {
        db.pragma_update(None, "journal_mode", mode.as_pragma())
            .map_err(map_err)?;
    }
    db.busy_timeout(Duration::from_millis(options.busy_timeout))
        .map_err(map_err)?;
    if let Some(synchronous) = options.synchronous {
        db.pragma_update(None, "synchronous", synchronous.as_pragma())
            .map_err(map_err)?;
    }
    Ok(())
}

fn open_pooled(filename: &str, options: &ResolvedSqliteOptions) -> Result<SqliteDb> {
    let db = connection_string::open(filename).map_err(map_err)?;
    configure(&db, options)?;
    Ok(db)
}

fn map_err(error: rusqlite::Error) -> DatasourceError {
    DatasourceError::message(error.to_string())
}
