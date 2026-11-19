use crate::command::{named_params, row_record};
use crate::connection::{map_err, MysqlConnection};
use crate::MysqlDriver;
use async_trait::async_trait;
use datasources::{statement_parameters, DatasourceError, Reader, Result, Row, Value};
use futures_core::Stream;
use mysql_async::prelude::Queryable;
use mysql_async::Conn;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;

pub(crate) struct MysqlReader {
    sets: Vec<Vec<Row>>,
    set: Mutex<usize>,
    index: Mutex<isize>,
    current: Mutex<Option<Row>>,
}

impl MysqlReader {
    pub(crate) async fn open(
        connection: &MysqlConnection,
        statements: Vec<String>,
        params: HashMap<String, Value>,
    ) -> Result<Self> {
        let mut slot = connection.lock_conn().await?;
        let conn = slot
            .as_mut()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let sets = result_sets(conn, &statements, &params).await?;
        drop(slot);
        Ok(Self {
            sets,
            set: Mutex::new(0),
            index: Mutex::new(-1),
            current: Mutex::new(None),
        })
    }
}

#[async_trait]
impl Reader for MysqlReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("reader").clone()
    }

    async fn close(&self) -> Result<()> {
        Ok(())
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

async fn result_sets(
    conn: &mut Conn,
    statements: &[String],
    params: &HashMap<String, Value>,
) -> Result<Vec<Vec<Row>>> {
    let mut sets = Vec::new();
    for sql in statements {
        let names = statement_parameters(sql);
        let used = params
            .iter()
            .filter(|(name, _)| {
                names
                    .iter()
                    .any(|found| found == name.trim_start_matches(['$', '@', ':']))
            })
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        sets.extend(iter_sets(conn, sql, &used).await?);
    }
    Ok(sets)
}

async fn iter_sets(
    conn: &mut Conn,
    sql: &str,
    params: &HashMap<String, Value>,
) -> Result<Vec<Vec<Row>>> {
    let mut result = conn
        .exec_iter(MysqlDriver::placeholders(sql), named_params(params))
        .await
        .map_err(map_err)?;
    let mut sets = Vec::new();
    while let Some(mut stream) = result.stream().await.map_err(map_err)? {
        let columns = stream.columns_ref().len();
        let mut rows = Vec::new();
        while let Some(row) = pull(Pin::new(&mut stream)).await.map_err(map_err)? {
            rows.push(row_record(row));
        }
        if columns > 0 {
            sets.push(rows);
        }
    }
    Ok(sets)
}

async fn pull<S, T, E>(mut stream: Pin<&mut S>) -> std::result::Result<Option<T>, E>
where
    S: Stream<Item = std::result::Result<T, E>> + Unpin + ?Sized,
{
    std::future::poll_fn(|cx| stream.as_mut().poll_next(cx))
        .await
        .transpose()
}
