use crate::command::{as_params, bind_pairs, from_sql};
use crate::connection::{map_err, SqliteConnection};
use crate::converters;
use async_trait::async_trait;
use datasources::{split_statements, DatasourceError, Reader, Result, Row, Schema, Value};
use rusqlite::Connection as SqliteDb;
use std::collections::HashMap;
use std::sync::Mutex;

pub(crate) struct SqliteReader {
    sets: Vec<Vec<Row>>,
    set: Mutex<usize>,
    index: Mutex<isize>,
    current: Mutex<Option<Row>>,
}

impl SqliteReader {
    pub(crate) fn open(
        connection: &SqliteConnection,
        sql: &str,
        params: &HashMap<String, Value>,
        pooled: bool,
        schema: Option<&Schema>,
    ) -> Result<Self> {
        let statements = split_statements(sql);
        let sets = if pooled {
            collect_pooled(connection, &statements, params, schema)?
        } else {
            collect_primary(connection, &statements, params, schema)?
        };
        Ok(Self {
            sets,
            set: Mutex::new(0),
            index: Mutex::new(-1),
            current: Mutex::new(None),
        })
    }
}

#[async_trait]
impl Reader for SqliteReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        let set = *self.set.lock().expect("reader");
        if set >= self.sets.len() {
            *self.current.lock().expect("reader") = None;
            return Ok(false);
        }
        let mut index = self.index.lock().expect("reader");
        *index += 1;
        let i = *index as usize;
        if i < self.sets[set].len() {
            *self.current.lock().expect("reader") = Some(self.sets[set][i].clone());
            Ok(true)
        } else {
            *self.current.lock().expect("reader") = None;
            Ok(false)
        }
    }

    async fn next_result(&self) -> Result<bool> {
        *self.current.lock().expect("reader") = None;
        *self.index.lock().expect("reader") = -1;
        let mut set = self.set.lock().expect("reader");
        *set += 1;
        Ok(*set < self.sets.len())
    }
}

fn collect_pooled(
    connection: &SqliteConnection,
    statements: &[String],
    params: &HashMap<String, Value>,
    schema: Option<&Schema>,
) -> Result<Vec<Vec<Row>>> {
    if connection.pinned {
        return collect_primary(connection, statements, params, schema);
    }
    match connection.pool() {
        Some(pool) => match pool.checkout()? {
            Some(checkout) => collect_sets(checkout.db(), statements, params, schema),
            None => collect_primary(connection, statements, params, schema),
        },
        None => collect_primary(connection, statements, params, schema),
    }
}

fn collect_primary(
    connection: &SqliteConnection,
    statements: &[String],
    params: &HashMap<String, Value>,
    schema: Option<&Schema>,
) -> Result<Vec<Vec<Row>>> {
    let db = connection.db.lock().unwrap();
    let conn = db
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
    collect_sets(conn, statements, params, schema)
}

fn collect_sets(
    conn: &SqliteDb,
    statements: &[String],
    params: &HashMap<String, Value>,
    schema: Option<&Schema>,
) -> Result<Vec<Vec<Row>>> {
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let pairs = bind_pairs(params);
    let mut sets = Vec::new();
    for sql in statements {
        let bound = as_params(&pairs);
        let mut stmt = conn.prepare(sql).map_err(map_err)?;
        let names: Vec<String> = stmt.column_names().into_iter().map(str::to_string).collect();
        let mut rows = stmt.query(bound.as_slice()).map_err(map_err)?;
        let mut set = Vec::new();
        while let Some(row) = rows.next().map_err(map_err)? {
            set.push(map_row(&row, &names, schema)?);
        }
        sets.push(set);
    }
    Ok(sets)
}

fn map_row(row: &rusqlite::Row<'_>, names: &[String], schema: Option<&Schema>) -> Result<Row> {
    let mut record = Row::new();
    for (index, name) in names.iter().enumerate() {
        record.insert(
            name.clone(),
            converted(schema, name, from_sql(row.get_ref(index).map_err(map_err)?))?,
        );
    }
    Ok(record)
}

fn converted(schema: Option<&Schema>, name: &str, cell: Value) -> Result<Value> {
    let Some(schema) = schema else {
        return Ok(cell);
    };
    let types = schema.types();
    let Some(ty) = types.get(name) else {
        return Ok(cell);
    };
    match converters::field(ty) {
        Some(converter) => converter.convert(&cell),
        None => Ok(cell),
    }
}
