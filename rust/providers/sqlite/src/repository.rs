use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::{
    identity_where_text, Command, Parameter, Query, Record, Repository, Response, Result, Where,
};
use std::sync::Arc;

pub struct SqliteRepository {
    connection: Arc<SqliteConnection>,
    pub table: String,
}

impl SqliteRepository {
    pub fn new(connection: Arc<SqliteConnection>, table: Option<String>) -> Self {
        Self {
            connection,
            table: table.unwrap_or_else(|| "items".to_string()),
        }
    }
}

#[async_trait]
impl Repository for SqliteRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let sql = select_sql(&self.table, query, true);
        let reader = SqliteCommand::new(self.connection.clone())
            .execute_reader(Some(&sql), None)
            .await?;
        if reader.next().await? {
            Ok(reader.current())
        } else {
            Ok(None)
        }
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let sql = select_sql(&self.table, query, false);
        let reader = SqliteCommand::new(self.connection.clone())
            .execute_reader(Some(&sql), None)
            .await?;
        datasources::collect_reader(&*reader).await
    }

    async fn add(&self, obj: &Record) -> Result<Response> {
        let keys: Vec<&String> = obj.keys().collect();
        let sql = if keys.is_empty() {
            format!("INSERT INTO {} DEFAULT VALUES", self.table)
        } else {
            let cols = keys
                .iter()
                .map(|k| (*k).as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let vals = keys
                .iter()
                .map(|k| format!("${k}"))
                .collect::<Vec<_>>()
                .join(", ");
            format!("INSERT INTO {} ({cols}) VALUES ({vals})", self.table)
        };
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let keys: Vec<&String> = obj.keys().collect();
        let assignments = if keys.is_empty() {
            "id = id".to_string()
        } else {
            keys.iter()
                .map(|k| format!("{k} = ${k}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let where_sql = by_where
            .map(|w| w.get_text())
            .unwrap_or_else(|| identity_where_text(obj));
        let sql = format!(
            "UPDATE {} SET {assignments} WHERE {where_sql}",
            self.table
        );
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let where_sql = by_where
            .map(|w| w.get_text())
            .unwrap_or_else(|| identity_where_text(obj));
        let sql = format!("DELETE FROM {} WHERE {where_sql}", self.table);
        SqliteCommand::new(self.connection.clone())
            .execute(Some(&sql), Some(&to_params(obj)))
            .await
    }
}

fn select_sql(table: &str, query: Option<&dyn Query>, single: bool) -> String {
    let where_sql = query
        .map(|q| format!(" WHERE {}", q.get_text()))
        .unwrap_or_default();
    let limit = if single { " LIMIT 1" } else { "" };
    format!("SELECT * FROM {table}{where_sql}{limit}")
}

fn to_params(obj: &Record) -> Vec<Parameter> {
    obj.iter()
        .map(|(k, v)| Parameter::new(k.clone(), v.clone()))
        .collect()
}
