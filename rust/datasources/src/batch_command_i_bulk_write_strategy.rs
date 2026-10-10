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
use crate::{DefaultStatementTextRenderer, Expression, Literal, Operator, Where};
use async_trait::async_trait;
use std::sync::{Arc, OnceLock};

#[derive(Default)]
pub struct BatchCommandIBulkWriteStrategy {
    command: OnceLock<Arc<dyn Command>>,
    options: OnceLock<CRUDRepositoryOptions>,
    connection: OnceLock<Arc<dyn Connection>>,
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

    fn statements(&self) -> Result<CRUDRepositoryStatements> {
        let built = CRUDRepositoryBuilder::parts(self.connection()?, self.options()?.clone())?;
        Ok(write_statements(&built.statements))
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

    fn insert(&self, item: &Row) -> Result<BulkCommand> {
        let record = self.reverse(item)?;
        let ids = self.ids()?.to_vec();
        let fields = self.fields(&record, |key, value| {
            !ids.iter().any(|id| id == key) || assigned(value)
        })?;
        let statement = self.statements()?.insert(&fields);
        Ok(BulkCommand {
            text: DefaultStatementTextRenderer::instance().render(&statement),
            parameters: statement.fields().to_vec(),
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

    async fn send(&self, commands: Vec<BulkCommand>) -> Result<Response> {
        if commands.is_empty() {
            return Ok(Response::new(Value::Null, 0, None));
        }
        let renamed: Vec<BulkCommand> = commands
            .iter()
            .enumerate()
            .map(|(index, command)| rename(command, index))
            .collect();
        let text = renamed
            .iter()
            .map(|command| command.text.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        let parameters: Vec<Parameter> = renamed
            .iter()
            .flat_map(|command| command.parameters.clone())
            .collect();
        self.command()?
            .execute_native(&Native::new(text), Some(&parameters))
            .await
    }
}

#[async_trait]
impl BulkWriteStrategy for BatchCommandIBulkWriteStrategy {
    fn set_command(&self, command: Arc<dyn Command>) {
        let _ = self.command.set(command);
    }

    fn set_options(&self, options: CRUDRepositoryOptions) {
        let _ = self.options.set(options);
    }

    fn set_connection(&self, connection: Arc<dyn Connection>) {
        let _ = self.connection.set(connection);
    }

    async fn create(&self, items: &mut [Row]) -> Result<Response> {
        let commands = items
            .iter()
            .map(|item| self.insert(item))
            .collect::<Result<Vec<_>>>()?;
        self.send(commands).await
    }

    async fn update(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response> {
        let commands = items
            .iter()
            .map(|item| self.update_command(item, by_where))
            .collect::<Result<Vec<_>>>()?;
        self.send(commands).await
    }

    async fn delete(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Response> {
        let commands = items
            .iter()
            .map(|item| self.delete_command(item, by_where))
            .collect::<Result<Vec<_>>>()?;
        self.send(commands).await
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

fn rename(command: &BulkCommand, index: usize) -> BulkCommand {
    let mut text = command.text.clone();
    let mut parameters = command.parameters.clone();
    let mut names: Vec<String> = parameters
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    names.sort();
    names.dedup();
    names.sort_by_key(|name| std::cmp::Reverse(name.len()));
    for name in names {
        let alias = format!("{name}__{index}");
        text = rename_token(&text, &name, &alias);
        for parameter in &mut parameters {
            if parameter.name == name {
                parameter.name = alias.clone();
            }
        }
    }
    BulkCommand { text, parameters }
}

fn rename_token(text: &str, name: &str, alias: &str) -> String {
    let token = format!("${name}");
    let next = format!("${alias}");
    let mut result = String::new();
    let mut index = 0;
    while let Some(at) = text[index..].find(&token) {
        let at = index + at;
        let end = at + token.len();
        let boundary = match text[end..].chars().next() {
            None => true,
            Some(ch) => !ch.is_ascii_alphanumeric() && ch != '_',
        };
        result.push_str(&text[index..at]);
        result.push_str(if boundary { &next } else { &token });
        index = end;
    }
    result.push_str(&text[index..]);
    result
}
