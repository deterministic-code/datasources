use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::command::Command;
use crate::connection::Connection;
use crate::crud_repository_builder::{write_statements, CRUDRepositoryBuilder};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource_error::DatasourceError;
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
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

pub struct StoredProcedureBulkWriteStrategy {
    create: String,
    update: String,
    delete: String,
    command: OnceLock<Arc<dyn Command>>,
    options: OnceLock<CRUDRepositoryOptions>,
    connection: OnceLock<Arc<dyn Connection>>,
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
            command: OnceLock::new(),
            options: OnceLock::new(),
            connection: OnceLock::new(),
        }
    }

    fn command(&self) -> Result<Arc<dyn Command>> {
        self.command
            .get()
            .cloned()
            .ok_or_else(|| DatasourceError::message("Command required"))
    }

    fn options(&self) -> Result<&CRUDRepositoryOptions> {
        self.options
            .get()
            .ok_or_else(|| DatasourceError::message("Options required"))
    }

    fn reverse(&self, item: &Row) -> Result<Row> {
        let options = self.options()?;
        let row = match &options.object_converter {
            Some(converter) => converter.reverse_convert(item)?,
            None => item.clone(),
        };
        match &options.row_converter {
            Some(converter) => converter.reverse_convert(&row),
            None => Ok(row),
        }
    }

    fn column(&self, property: &str) -> String {
        self.options()
            .ok()
            .and_then(|options| options.property_mappings.as_ref())
            .map(|mapper| mapper.map(property.to_string()))
            .unwrap_or_else(|| property.to_string())
    }

    fn row_of(&self, item: &Row) -> Result<Row> {
        let stored = self.reverse(item)?;
        let connection = self
            .connection
            .get()
            .cloned()
            .ok_or_else(|| DatasourceError::message("Connection required"))?;
        let built = CRUDRepositoryBuilder::parts(connection, self.options()?.clone())?;
        let statements = write_statements(&built.statements);
        let mut row = Row::new();
        for name in statements.read_columns() {
            let value = stored
                .iter()
                .find(|(key, _)| self.column(key) == *name)
                .map(|(_, value)| value.clone())
                .unwrap_or(Value::Null);
            row.insert(name.clone(), value);
        }
        Ok(row)
    }

    async fn call(&self, name: &str, items: &[Row]) -> Result<Response> {
        if items.is_empty() {
            return Ok(Response::new(Value::Null, 0, None));
        }
        let rows = items
            .iter()
            .map(|item| self.row_of(item))
            .collect::<Result<Vec<_>>>()?;
        let payload = Parameter::new("payload", json(&rows));
        let statement = ExecuteStoredProcedureStatement::new(name, &[payload.clone()]);
        self.command()?
            .execute_statement(&statement, Some(&[payload]))
            .await
    }
}

#[async_trait]
impl BulkWriteStrategy for StoredProcedureBulkWriteStrategy {
    fn attach(
        &self,
        command: Arc<dyn Command>,
        options: CRUDRepositoryOptions,
        connection: Arc<dyn Connection>,
    ) {
        let _ = self.command.set(command);
        let _ = self.options.set(options);
        let _ = self.connection.set(connection);
    }

    fn persist_children(&self) -> bool {
        true
    }

    async fn create(&self, items: &mut [Row]) -> Result<Response> {
        self.call(&self.create, items).await
    }

    async fn update(&self, items: &[Row], _by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.call(&self.update, items).await
    }

    async fn delete(&self, items: &[Row], _by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.call(&self.delete, items).await
    }
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
