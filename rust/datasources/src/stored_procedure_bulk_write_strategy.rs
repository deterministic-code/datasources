use crate::bulk_crud_repository::BulkCRUDRepository;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::i_where::IWhere;
use crate::parameter::Parameter;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::ExecuteStoredProcedureStatement;
use async_trait::async_trait;
use base64::Engine;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use uuid::Uuid;

pub struct StoredProcedureBulkWriteStrategy {
    create: String,
    update: String,
    delete: String,
}

impl StoredProcedureBulkWriteStrategy {
    pub fn new(
        create: impl Into<String>,
        update: impl Into<String>,
        delete: impl Into<String>,
    ) -> Self {
        Self {
            create: create.into(),
            update: update.into(),
            delete: delete.into(),
        }
    }
}

#[async_trait]
impl BulkWriteStrategy for StoredProcedureBulkWriteStrategy {
    async fn create(&self, items: &mut [Row], repo: &BulkCRUDRepository) -> Result<Response> {
        call(repo, &self.create, items).await
    }

    async fn update(
        &self,
        items: &[Row],
        _by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        call(repo, &self.update, items).await
    }

    async fn delete(
        &self,
        items: &[Row],
        _by_where: Option<&dyn IWhere>,
        repo: &BulkCRUDRepository,
    ) -> Result<Response> {
        call(repo, &self.delete, items).await
    }
}

async fn call(repo: &BulkCRUDRepository, name: &str, items: &[Row]) -> Result<Response> {
    if items.is_empty() {
        return Ok(Response::new(Value::Null, 0));
    }
    let payload = Parameter::new("payload", json(&repo.rows(items)?));
    let statement = ExecuteStoredProcedureStatement::new(name, &[payload.clone()]);
    repo.command()
        .execute_statement(&statement, Some(&[payload]))
        .await
}

fn json(rows: &[Row]) -> String {
    let objects: Vec<String> = rows.iter().map(object).collect();
    format!("[{}]", objects.join(","))
}

fn object(row: &Row) -> String {
    let fields: BTreeMap<&str, &Value> = row
        .iter()
        .map(|(key, value)| (key.as_str(), value))
        .collect();
    let body: Vec<String> = fields
        .into_iter()
        .map(|(key, value)| format!("{}:{}", quote(key), scalar(value)))
        .collect();
    format!("{{{}}}", body.join(","))
}

fn scalar(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(flag) => if *flag { "true" } else { "false" }.to_string(),
        Value::Int(number) => number.to_string(),
        Value::Float(number) => number.to_string(),
        Value::Text(text) => quote(text),
        Value::DateTime(date) => quote(&round_trip(*date)),
        Value::Uuid(id) => quote(&uuid_text(id)),
        Value::Decimal(number) => quote(&decimal_text(number)),
        Value::Binary(bytes) => quote(&base64::engine::general_purpose::STANDARD.encode(bytes)),
    }
}

fn round_trip(date: DateTime<Utc>) -> String {
    format!(
        "{}.{:07}Z",
        date.format("%Y-%m-%dT%H:%M:%S"),
        date.timestamp_subsec_nanos() / 100
    )
}

fn uuid_text(id: &Uuid) -> String {
    id.to_string()
}

fn decimal_text(number: &Decimal) -> String {
    number.to_string()
}

fn quote(text: &str) -> String {
    let mut result = String::from("\"");
    for ch in text.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\u{0008}' => result.push_str("\\b"),
            '\u{000c}' => result.push_str("\\f"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            ch if (ch as u32) < 0x20 => result.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => result.push(ch),
        }
    }
    result.push('"');
    result
}
