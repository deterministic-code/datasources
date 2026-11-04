use crate::bind::{param_types, to_postgres};
use crate::command::{box_value, row_record};
use crate::connection::{map_err, PostgresConnection};
use async_trait::async_trait;
use datasources::{DatasourceError, Reader, Result, Row, Value};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio_postgres::types::ToSql;

pub(crate) struct PostgresReader {
    sets: Vec<Vec<Row>>,
    set: Mutex<usize>,
    index: Mutex<isize>,
    current: Mutex<Option<Row>>,
}

impl PostgresReader {
    pub(crate) async fn open(
        connection: &PostgresConnection,
        statements: Vec<String>,
        params: HashMap<String, Value>,
    ) -> Result<Self> {
        let slot = connection.lock_client().await?;
        let client = slot
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Connection is not open"))?;
        let mut sets = Vec::new();
        for sql in statements {
            let (text, values) = to_postgres(&sql, &params);
            let boxed: Vec<Box<dyn ToSql + Sync + Send>> = values.iter().map(box_value).collect();
            let refs: Vec<&(dyn ToSql + Sync)> = boxed.iter().map(|value| value.as_ref() as _).collect();
            let statement = client
                .prepare_typed(&text, &param_types(&values))
                .await
                .map_err(map_err)?;
            let rows = client.query(&statement, &refs).await.map_err(map_err)?;
            sets.push(rows.iter().map(row_record).collect());
        }
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
impl Reader for PostgresReader {
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
