use crate::command::{as_params, bind_pairs, from_sql};
use crate::converters::BoolConverter;
use crate::connection::{map_err, SqliteConnection};
use async_trait::async_trait;
use datasources::{split_statements, DatasourceError, Reader, Result, Row, TypeConverter, Value};
use rusqlite::Connection as SqliteDb;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{mpsc, Mutex};
use std::thread::{self, JoinHandle};

enum Ask {
    Next,
    NextResult,
    Shutdown,
}

enum Answer {
    Ready,
    Row(Option<Row>),
    More(bool),
    Err(String),
}

struct SqliteCursor {
    tx: Mutex<Option<mpsc::Sender<Ask>>>,
    thread: Mutex<Option<JoinHandle<()>>>,
    rx: Mutex<mpsc::Receiver<Answer>>,
    current: Mutex<Option<Row>>,
}

impl SqliteCursor {
    fn shutdown(&self) {
        if let Some(tx) = self.tx.lock().expect("reader").take() {
            let _ = tx.send(Ask::Shutdown);
        }
        if let Some(thread) = self.thread.lock().expect("reader").take() {
            let _ = thread.join();
        }
    }

    fn ask(&self, ask: Ask) -> Result<Answer> {
        let guard = self.tx.lock().expect("reader");
        let tx = guard
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Reader is closed"))?;
        tx.send(ask)
            .map_err(|_| DatasourceError::message("Reader is closed"))?;
        drop(guard);
        self.rx
            .lock()
            .expect("reader")
            .recv()
            .map_err(|_| DatasourceError::message("Reader is closed"))
    }
}

impl Drop for SqliteCursor {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub(crate) struct SqliteReader {
    cursor: SqliteCursor,
}

impl SqliteReader {
    pub(crate) fn open(
        connection: &SqliteConnection,
        sql: &str,
        params: &HashMap<String, Value>,
    ) -> Result<Self> {
        let statements = split_statements(sql);
        let conn = connection
            .db
            .lock()
            .expect("sqlite")
            .take()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let slot = connection.db.clone();
        let opened = connection.opened.clone();
        let (ask_tx, ask_rx) = mpsc::channel();
        let (ans_tx, ans_rx) = mpsc::channel();
        let params = params.clone();
        let thread = thread::spawn(move || {
            let conn_ref = Box::leak(Box::new(conn));
            let _ = catch_unwind(AssertUnwindSafe(|| {
                if let Err(error) = drive(conn_ref, &statements, &params, &ask_rx, &ans_tx) {
                    let _ = ans_tx.send(Answer::Err(error.to_string()));
                }
            }));
            let conn = unsafe { Box::from_raw(conn_ref) };
            restore(&slot, &opened, *conn);
        });
        match ans_rx.recv() {
            Ok(Answer::Ready) => Ok(Self {
                cursor: SqliteCursor {
                    tx: Mutex::new(Some(ask_tx)),
                    thread: Mutex::new(Some(thread)),
                    rx: Mutex::new(ans_rx),
                    current: Mutex::new(None),
                },
            }),
            Ok(Answer::Err(error)) => {
                let _ = ask_tx.send(Ask::Shutdown);
                let _ = thread.join();
                Err(DatasourceError::message(error))
            }
            _ => {
                let _ = ask_tx.send(Ask::Shutdown);
                let _ = thread.join();
                Err(DatasourceError::message("Reader failed to open"))
            }
        }
    }
}

#[async_trait]
impl Reader for SqliteReader {
    fn current(&self) -> Option<Row> {
        self.cursor.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        match self.cursor.ask(Ask::Next)? {
            Answer::Row(row) => {
                let found = row.is_some();
                *self.cursor.current.lock().expect("reader") = row;
                Ok(found)
            }
            Answer::Err(error) => Err(DatasourceError::message(error)),
            _ => Err(DatasourceError::message("Reader failed")),
        }
    }

    async fn next_result(&self) -> Result<bool> {
        *self.cursor.current.lock().expect("reader") = None;
        match self.cursor.ask(Ask::NextResult)? {
            Answer::More(more) => Ok(more),
            Answer::Err(error) => Err(DatasourceError::message(error)),
            _ => Err(DatasourceError::message("Reader failed")),
        }
    }
}

fn restore(slot: &Mutex<Option<SqliteDb>>, opened: &Mutex<bool>, conn: SqliteDb) {
    let mut db = slot.lock().expect("sqlite");
    if db.is_none() && *opened.lock().expect("sqlite") {
        *db = Some(conn);
    }
}

fn drive(
    conn: &SqliteDb,
    statements: &[String],
    params: &HashMap<String, Value>,
    ask_rx: &mpsc::Receiver<Ask>,
    ans_tx: &mpsc::Sender<Answer>,
) -> Result<()> {
    let pairs = bind_pairs(params);
    if statements.is_empty() {
        let _ = ans_tx.send(Answer::Ready);
        return idle(ask_rx, ans_tx);
    }
    let mut index = 0;
    loop {
        let bound = as_params(&pairs);
        let mut stmt = conn.prepare(&statements[index]).map_err(map_err)?;
        let names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect();
        let declared: Vec<Option<String>> = stmt
            .columns()
            .into_iter()
            .map(|column| column.decl_type().map(ToOwned::to_owned))
            .collect();
        let mut rows = stmt.query(bound.as_slice()).map_err(map_err)?;
        if index == 0 {
            let _ = ans_tx.send(Answer::Ready);
        }
        loop {
            match ask_rx.recv() {
                Ok(Ask::Next) => {
                    let row = rows.next().map_err(map_err)?;
                    let mapped = match row {
                        Some(row) => Some(map_row(&row, &names, &declared)?),
                        None => None,
                    };
                    let _ = ans_tx.send(Answer::Row(mapped));
                }
                Ok(Ask::NextResult) => break,
                _ => return Ok(()),
            }
        }
        index += 1;
        if index >= statements.len() {
            let _ = ans_tx.send(Answer::More(false));
            return idle(ask_rx, ans_tx);
        }
        let _ = ans_tx.send(Answer::More(true));
    }
}

fn idle(ask_rx: &mpsc::Receiver<Ask>, ans_tx: &mpsc::Sender<Answer>) -> Result<()> {
    loop {
        match ask_rx.recv() {
            Ok(Ask::Next) => {
                let _ = ans_tx.send(Answer::Row(None));
            }
            Ok(Ask::NextResult) => {
                let _ = ans_tx.send(Answer::More(false));
            }
            _ => return Ok(()),
        }
    }
}

fn map_row(row: &rusqlite::Row<'_>, names: &[String], declared: &[Option<String>]) -> Result<Row> {
    let converter = BoolConverter;
    let mut record = Row::new();
    for (index, name) in names.iter().enumerate() {
        let value = from_sql(row.get_ref(index).map_err(map_err)?);
        let value = match declared.get(index).and_then(Option::as_deref) {
            Some(kind) if kind.eq_ignore_ascii_case("BOOLEAN") => converter.convert(&value)?,
            _ => value,
        };
        record.insert(name.clone(), value);
    }
    Ok(record)
}
