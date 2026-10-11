use crate::bulk_command::BulkCommand;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::command::Command;
use crate::connection::Connection;
use crate::crud_repository_builder::{write_statements, CRUDRepositoryBuilder};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::crud_repository_statements::CRUDRepositoryStatements;
use crate::datasource_error::DatasourceError;
use crate::i_where::IWhere;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{
    BulkDeleteStatement, BulkInsertStatement, BulkUpdateRow, BulkUpdateStatement,
    DefaultStatementTextRenderer, Expression, Literal, Operator, Statement, Where,
};
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};

const PARAMETER_BUDGET: usize = 8000;
const MAX_ROWS: usize = 250;

struct Inserted {
    table: String,
    fields: Vec<Parameter>,
}

struct Updated {
    table: String,
    fields: Vec<Parameter>,
    ids: Vec<Parameter>,
}

struct Deleted {
    table: String,
    ids: Vec<Parameter>,
}

#[derive(Default)]
pub struct BatchCommandIBulkWriteStrategy {
    command: OnceLock<Arc<dyn Command>>,
    options: OnceLock<CRUDRepositoryOptions>,
    connection: OnceLock<Arc<dyn Connection>>,
    statements: OnceLock<CRUDRepositoryStatements>,
}

impl BatchCommandIBulkWriteStrategy {
    pub fn new() -> Self {
        Self::default()
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

    fn connection(&self) -> Result<Arc<dyn Connection>> {
        self.connection
            .get()
            .cloned()
            .ok_or_else(|| DatasourceError::message("Connection required"))
    }

    fn statements(&self) -> Result<&CRUDRepositoryStatements> {
        if let Some(statements) = self.statements.get() {
            return Ok(statements);
        }
        let built = CRUDRepositoryBuilder::parts(self.connection()?, self.options()?.clone())?;
        let _ = self.statements.set(write_statements(&built.statements));
        self.statements
            .get()
            .ok_or_else(|| DatasourceError::message("Statements required"))
    }

    fn ids(&self) -> Result<&[String]> {
        Ok(&self.options()?.ids)
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

    fn cell(&self, property: &str, value: &Value, fields_applied: bool) -> Result<Value> {
        if fields_applied {
            return Ok(value.clone());
        }
        match self
            .options()?
            .row_converter
            .as_ref()
            .and_then(|converter| converter.fields.get(property))
        {
            Some(field) => field.reverse_convert(value),
            None => Ok(value.clone()),
        }
    }

    fn fields(&self, record: &Row, include: impl Fn(&str, &Value) -> bool) -> Result<Vec<Parameter>> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, value)| include(key, value) && !matches!(value, Value::Null))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| Ok(Parameter::new(name.clone(), self.cell(&name, &value, true)?)))
            .collect()
    }

    fn id_where(&self, record: &Row) -> Result<Where> {
        let children = self
            .ids()?
            .iter()
            .map(|col| {
                let value = record.get(col).ok_or_else(|| {
                    DatasourceError::message(format!("Missing Id value \"{col}\""))
                })?;
                Ok(Expression::from(Operator::equals(col.clone(), value.clone())))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Where::new(and_all(children)))
    }

    fn mapped_insert(&self, item: &Row) -> Result<Inserted> {
        let record = self.reverse(item)?;
        let ids = self.ids()?.to_vec();
        let fields = self.fields(&record, |key, value| {
            !ids.iter().any(|id| id == key) || assigned(value)
        })?;
        let statement = self.statements()?.insert(&fields);
        Ok(Inserted {
            table: statement.table().to_string(),
            fields: statement.fields().to_vec(),
        })
    }

    fn mapped_update(&self, item: &Row) -> Result<Updated> {
        let record = self.reverse(item)?;
        let ids = self.ids()?.to_vec();
        let fields = self.fields(&record, |key, _| !ids.iter().any(|id| id == key))?;
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let clause = self.id_where(&record)?;
        let statement = self.statements()?.update(&fields, &clause, &|name, value| {
            self.cell(name, value, true)
        })?;
        Ok(Updated {
            table: statement.table().to_string(),
            fields: statement.fields().to_vec(),
            ids: statement
                .where_clause()
                .map(|clause| clause.parameters())
                .unwrap_or_default(),
        })
    }

    fn mapped_delete(&self, item: &Row) -> Result<Deleted> {
        let record = self.reverse(item)?;
        let clause = self.id_where(&record)?;
        let statement = self.statements()?.delete(&clause, &|name, value| {
            self.cell(name, value, true)
        })?;
        Ok(Deleted {
            table: statement.table().to_string(),
            ids: statement
                .where_clause()
                .map(|clause| clause.parameters())
                .unwrap_or_default(),
        })
    }

    fn update_command(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<BulkCommand> {
        let record = self.reverse(item)?;
        let ids = self.ids()?.to_vec();
        let fields = self.fields(&record, |key, _| !ids.iter().any(|id| id == key))?;
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let owned;
        let clause: &dyn IWhere = match by_where {
            Some(clause) => clause,
            None => {
                owned = self.id_where(&record)?;
                &owned
            }
        };
        let applied = by_where.is_none();
        let statement = self.statements()?.update(&fields, clause, &|name, value| {
            self.cell(name, value, applied)
        })?;
        Ok(BulkCommand {
            text: DefaultStatementTextRenderer::instance().render(&statement),
            parameters: statement.parameters(),
        })
    }

    fn delete_command(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<BulkCommand> {
        let record = self.reverse(item)?;
        let owned;
        let clause: &dyn IWhere = match by_where {
            Some(clause) => clause,
            None => {
                owned = self.id_where(&record)?;
                &owned
            }
        };
        let applied = by_where.is_none();
        let statement = self.statements()?.delete(clause, &|name, value| {
            self.cell(name, value, applied)
        })?;
        let parameters = statement
            .where_clause()
            .map(|clause| clause.parameters())
            .unwrap_or_default();
        let rendered = DefaultStatementTextRenderer::instance().render(&statement);
        let parameters = if parameters.is_empty() && rendered.contains('$') {
            self.fields(&record, |_, _| true)?
        } else {
            parameters
        };
        Ok(BulkCommand {
            text: rendered,
            parameters,
        })
    }

    async fn execute(&self, statement: &dyn Statement, parameters: Vec<Parameter>) -> Result<Response> {
        self.command()?
            .execute_statement(statement, Some(&parameters))
            .await
    }

    async fn chunked<T, F, Fut>(&self, rows: &[T], width: usize, write: F) -> Result<Response>
    where
        F: Fn(&[T]) -> Fut,
        Fut: std::future::Future<Output = Result<Response>>,
    {
        let size = (PARAMETER_BUDGET / width.max(1)).max(1).min(MAX_ROWS);
        let mut rows_affected = 0;
        for chunk in rows.chunks(size) {
            rows_affected += write(chunk).await?.rows_affected;
        }
        Ok(Response::new(Value::Null, rows_affected, None))
    }

    async fn send_each(&self, commands: Vec<BulkCommand>) -> Result<Response> {
        let mut rows_affected = 0;
        for command in commands {
            rows_affected += self
                .command()?
                .execute_native(&Native::new(command.text), Some(&command.parameters))
                .await?
                .rows_affected;
        }
        Ok(Response::new(Value::Null, rows_affected, None))
    }

    async fn write_creates(&self, items: &[Row]) -> Result<Response> {
        if items.is_empty() {
            return Ok(Response::new(Value::Null, 0, None));
        }
        let rows = items
            .iter()
            .map(|item| self.mapped_insert(item))
            .collect::<Result<Vec<_>>>()?;
        let columns = columns(rows.iter().map(|row| row.fields.iter()));
        let table = rows[0].table.clone();
        self.chunked(&rows, columns.len(), |chunk| {
            let table = table.clone();
            let columns = columns.clone();
            let fields = chunk.iter().map(|row| row.fields.clone()).collect::<Vec<_>>();
            async move {
                let statement = BulkInsertStatement::new(&table, &columns, fields);
                let parameters = statement.parameters();
                self.execute(&statement, parameters).await
            }
        })
        .await
    }

    async fn write_updates(&self, items: &[Row]) -> Result<Response> {
        if items.is_empty() {
            return Ok(Response::new(Value::Null, 0, None));
        }
        let rows = items
            .iter()
            .map(|item| self.mapped_update(item))
            .collect::<Result<Vec<_>>>()?;
        let set = columns(rows.iter().map(|row| row.fields.iter()));
        let ids = columns(rows.iter().map(|row| row.ids.iter()));
        let table = rows[0].table.clone();
        self.chunked(&rows, set.len() + ids.len(), |chunk| {
            let table = table.clone();
            let set = set.clone();
            let ids = ids.clone();
            let updated = chunk
                .iter()
                .map(|row| BulkUpdateRow {
                    fields: row.fields.clone(),
                    ids: row.ids.clone(),
                })
                .collect::<Vec<_>>();
            async move {
                let statement = BulkUpdateStatement::new(&table, &set, &ids, updated);
                let parameters = statement.parameters();
                self.execute(&statement, parameters).await
            }
        })
        .await
    }

    async fn write_deletes(&self, items: &[Row]) -> Result<Response> {
        if items.is_empty() {
            return Ok(Response::new(Value::Null, 0, None));
        }
        let rows = items
            .iter()
            .map(|item| self.mapped_delete(item))
            .collect::<Result<Vec<_>>>()?;
        let ids = columns(rows.iter().map(|row| row.ids.iter()));
        let table = rows[0].table.clone();
        self.chunked(&rows, ids.len(), |chunk| {
            let table = table.clone();
            let ids = ids.clone();
            let deleted = chunk.iter().map(|row| row.ids.clone()).collect::<Vec<_>>();
            async move {
                let statement = BulkDeleteStatement::new(&table, &ids, deleted);
                let parameters = statement.parameters();
                self.execute(&statement, parameters).await
            }
        })
        .await
    }
}

#[async_trait]
impl BulkWriteStrategy for BatchCommandIBulkWriteStrategy {
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
        self.write_creates(items).await
    }

    async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response> {
        match by_where {
            Some(_) => {
                let commands = items
                    .iter()
                    .map(|item| self.update_command(item, by_where))
                    .collect::<Result<Vec<_>>>()?;
                self.send_each(commands).await
            }
            None => self.write_updates(items).await,
        }
    }

    async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response> {
        match by_where {
            Some(_) => {
                let commands = items
                    .iter()
                    .map(|item| self.delete_command(item, by_where))
                    .collect::<Result<Vec<_>>>()?;
                self.send_each(commands).await
            }
            None => self.write_deletes(items).await,
        }
    }
}

fn assigned(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Text(text) => !text.is_empty(),
        Value::Int(0) => false,
        _ => true,
    }
}

fn and_all(parts: Vec<Expression>) -> Expression {
    parts
        .into_iter()
        .reduce(|left, right| Operator::and(left, right).into())
        .unwrap_or_else(|| {
            Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }
            .into()
        })
}

fn columns<'a>(rows: impl IntoIterator<Item = impl IntoIterator<Item = &'a Parameter>>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut columns = Vec::new();
    for field in rows.into_iter().flatten() {
        if seen.insert(field.name.clone()) {
            columns.push(field.name.clone());
        }
    }
    columns
}

