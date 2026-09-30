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
        let names = qualified(&statements[index], &stmt.column_names());
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

fn qualified(sql: &str, names: &[&str]) -> Vec<String> {
    let mut counts = std::collections::HashMap::<&str, usize>::new();
    for name in names {
        *counts.entry(name).or_insert(0) += 1;
    }
    let fields = select_items(sql);
    names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let field = fields.get(index).map(String::as_str).map(str::trim);
            match (counts[name] > 1, field) {
                (true, Some(field)) if field.rsplit('.').next() == Some(*name) => field.to_string(),
                _ => (*name).to_string(),
            }
        })
        .collect()
}

fn select_items(sql: &str) -> Vec<String> {
    let upper = sql.to_ascii_uppercase();
    let Some(select_at) = upper.find("SELECT") else {
        return Vec::new();
    };
    let mut rest = &sql[select_at + 6..];
    if rest.trim_start().len() != rest.len() {
        rest = rest.trim_start();
    }
    if rest.to_ascii_uppercase().starts_with("DISTINCT") {
        rest = rest[8..].trim_start();
    }
    let end = top_from(rest);
    split_commas(&rest[..end])
}

fn top_from(sql: &str) -> usize {
    let mut depth = 0;
    let bytes = sql.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'(' => depth += 1,
            b')' if depth > 0 => depth -= 1,
            b'F' | b'f' if depth == 0 && from_word(sql, index) => return index,
            _ => {}
        }
        index += 1;
    }
    sql.len()
}

fn from_word(sql: &str, index: usize) -> bool {
    let end = index + 4;
    if end > sql.len() || !sql[index..end].eq_ignore_ascii_case("FROM") {
        return false;
    }
    let before = index == 0 || !sql.as_bytes()[index - 1].is_ascii_alphanumeric();
    let after = end == sql.len() || !sql.as_bytes()[end].is_ascii_alphanumeric();
    before && after
}

fn split_commas(list: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut start = 0;
    let mut depth = 0;
    for (index, ch) in list.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            ',' if depth == 0 => {
                items.push(list[start..index].trim().to_string());
                start = index + 1;
            }
            _ => {}
        }
    }
    let tail = list[start..].trim();
    if !tail.is_empty() {
        items.push(tail.to_string());
    }
    items
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
