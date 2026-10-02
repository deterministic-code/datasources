use crate::connection::MemoryConnection;
use crate::filter::{matches, parse_filter};
use async_trait::async_trait;
use datasources::{
    parameters_to_map, ArrayReader, Command, CommandMiddleware, CommandMiddlewareHost,
    DatasourceError, IWhere, Native, Parameter, Reader, Response, Result, Row, SelectInsertedIdStatement,
    Statement, ExpressionEvaluator, StatementKind, Value, Where,
};
use std::sync::Arc;

pub struct MemoryCommand {
    host: CommandMiddlewareHost,
    connection: Arc<MemoryConnection>,
}

impl MemoryCommand {
    pub fn new(connection: Arc<MemoryConnection>) -> Self {
        Self {
            host: CommandMiddlewareHost::new(),
            connection,
        }
    }
}

#[async_trait]
impl Command for MemoryCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        self.execute_reader(text.text(), parameters).await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        self.execute(text.text(), parameters).await
    }

    async fn execute_reader(
        &self,
        text: &str,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let mut filter = parse_filter(Some(&query))?;
                    filter.extend(parameters_to_map(params.as_deref()));
                    let rows = connection.collection(|items| {
                        items
                            .iter()
                            .filter(|row| matches(row, &filter))
                            .cloned()
                            .collect::<Vec<_>>()
                    });
                    Ok((
                        Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                        rows,
                    ))
                }
            })
            .await
    }

    async fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        if text.trim().is_empty() {
            return Err(DatasourceError::message("empty command"));
        }
        let connection = self.connection.clone();
        self.host
            .run(text, parameters, move |query, params| {
                let connection = connection.clone();
                async move {
                    connection.ensure_open()?;
                    let op = query.trim().to_ascii_lowercase();
                    let params = parameters_to_map(params.as_deref());
                    match op.as_str() {
                        "insert" => {
                            let mut row = params.clone();
                            let id = inserted_id(&row, &connection);
                            row.insert("id".into(), id.clone());
                            connection.collection(|items| items.push(row));
                            connection.remember_inserted(&connection.collection_name(), id.clone());
                            Ok((Response::new(id, 1, None), vec![]))
                        }
                        "update" => {
                            let mut obj = params.clone();
                            let where_text = obj.remove("$where").and_then(|v| match v {
                                Value::Text(text) => Some(text),
                                _ => None,
                            });
                            let filter = if let Some(text) = where_text {
                                parse_filter(Some(&text))?
                            } else {
                                obj.clone()
                            };
                            let affected = connection.collection(|items| {
                                let mut count = 0i64;
                                for item in items.iter_mut() {
                                    if matches(item, &filter) {
                                        item.extend(obj.clone());
                                        count += 1;
                                    }
                                }
                                count
                            });
                            Ok((Response::new(Value::Null, affected, None), vec![]))
                        }
                        "delete" => {
                            let filter = params.clone();
                            let affected = connection.collection(|items| {
                                let before = items.len();
                                items.retain(|row| !matches(row, &filter));
                                (before - items.len()) as i64
                            });
                            Ok((Response::new(Value::Null, affected, None), vec![]))
                        }
                        _ => Err(DatasourceError::message(format!(
                            "Unsupported Memory command: {op}"
                        ))),
                    }
                }
            })
            .await
    }

    async fn execute_reader_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        if let Some(inserted) = statement.as_any().downcast_ref::<SelectInsertedIdStatement>() {
            let connection = self.connection.clone();
            let table = inserted.table.clone();
            let column = inserted.column.clone();
            let text = format!("{table}.{column}");
            return self
                .host
                .run(&text, parameters, move |_query, _params| {
                    let connection = connection.clone();
                    let table = table.clone();
                    let column = column.clone();
                    async move {
                        connection.ensure_open()?;
                        let id = connection.inserted(&table);
                        let rows = match id {
                            Value::Null => vec![],
                            value => {
                                let mut row = Row::new();
                                row.insert(column, value);
                                vec![row]
                            }
                        };
                        Ok((
                            Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                            rows,
                        ))
                    }
                })
                .await;
        }
        let text = datasources::DefaultStatementTextRenderer::instance().render(statement);
        let connection = self.connection.clone();
        let table = table_of(statement);
        if let Some(condition) = statement
            .predicate()
            .and_then(|predicate| predicate.as_any().downcast_ref::<Where>())
            .map(|clause| clause.condition.clone())
        {
            return self
                .host
                .run(&text, parameters, move |_query, _params| {
                    let connection = connection.clone();
                    let condition = condition.clone();
                    let table = table.clone();
                    async move {
                        connection.ensure_open()?;
                        let evaluator = ExpressionEvaluator;
                        let rows = connection.with_rows(table.as_deref(), false, |items| {
                            items
                                .iter()
                                .filter(|row| {
                                    evaluator.evaluate(&condition, row) == Value::Bool(true)
                                })
                                .cloned()
                                .collect::<Vec<_>>()
                        });
                        Ok((
                            Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                            rows,
                        ))
                    }
                })
                .await;
        }
        let where_text = statement
            .predicate()
            .map(|predicate| {
                datasources::DefaultStatementTextRenderer::instance().render_where(predicate)
            })
            .unwrap_or_default();
        if !where_text.trim_start().starts_with('{')
            && !text.trim_start().starts_with('{')
            && !text.trim().is_empty()
        {
            return self
                .host
                .run(&text, parameters, move |_query, _params| {
                    let connection = connection.clone();
                    let table = table.clone();
                    async move {
                        connection.ensure_open()?;
                        let rows =
                            connection.with_rows(table.as_deref(), false, |items| items.clone());
                        Ok((
                            Box::new(ArrayReader::new(rows.clone())) as Box<dyn Reader>,
                            rows,
                        ))
                    }
                })
                .await;
        }
        self.execute_reader(&filter_text(statement), parameters)
            .await
    }

    async fn execute_statement(
        &self,
        statement: &dyn Statement,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let table = table_of(statement);
        match statement.kind() {
            StatementKind::Insert => {
                self.write_row("insert", table.as_deref(), true, None, parameters)
                    .await
            }
            StatementKind::Update => {
                self.write_row(
                    "update",
                    table.as_deref(),
                    false,
                    statement.predicate(),
                    parameters,
                )
                .await
            }
            StatementKind::Delete => {
                self.write_row(
                    "delete",
                    table.as_deref(),
                    false,
                    statement.predicate(),
                    parameters,
                )
                .await
            }
            _ => {
                self.execute(
                    &datasources::DefaultStatementTextRenderer::instance().render(statement),
                    parameters,
                )
                .await
            }
        }
    }
}

fn next_id(row: &Row, rows: &[Row]) -> Value {
    if let Some(id @ Value::Int(_)) = row.get("id") {
        return id.clone();
    }
    Value::Int(
        rows.iter()
            .filter_map(|item| match item.get("id") {
                Some(Value::Int(id)) => Some(*id),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + 1,
    )
}

impl MemoryCommand {
    async fn write_row(
        &self,
        op: &str,
        table: Option<&str>,
        create: bool,
        predicate: Option<&dyn IWhere>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let connection = self.connection.clone();
        let table = table.map(str::to_string);
        let op_text = op.to_string();
        let predicate_text = predicate.map(|item| {
            datasources::DefaultStatementTextRenderer::instance().render_where(item)
        });
        let condition = predicate
            .and_then(|item| item.as_any().downcast_ref::<Where>())
            .map(|clause| clause.condition.clone());
        let label = op_text.clone();
        let where_names = predicate
            .map(|item| {
                item.parameters()
                    .into_iter()
                    .map(|parameter| parameter.name)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.host
            .run(&label, parameters, move |_query, params| {
                let connection = connection.clone();
                let table = table.clone();
                let op = op_text.clone();
                let predicate_text = predicate_text.clone();
                let condition = condition.clone();
                let where_names = where_names.clone();
                async move {
                    connection.ensure_open()?;
                    let params = parameters_to_map(params.as_deref());
                    if op == "insert" {
                        let id = connection
                            .with_rows(table.as_deref(), create, |items| next_id(&params, items));
                        connection.with_rows(table.as_deref(), create, |items| {
                            let mut row = params.clone();
                            row.insert("id".into(), id.clone());
                            items.push(row);
                        });
                        let stored = table
                            .clone()
                            .unwrap_or_else(|| connection.collection_name());
                        connection.remember_inserted(&stored, id.clone());
                        return Ok((Response::new(id, 1, None), vec![]));
                    }
                    if op == "update" {
                        let affected = connection.with_rows(table.as_deref(), false, |items| {
                            apply_update(
                                items,
                                &params,
                                predicate_text.as_deref(),
                                condition.as_ref(),
                                &where_names,
                            )
                        });
                        return Ok((Response::new(Value::Null, affected, None), vec![]));
                    }
                    let affected = connection.with_rows(table.as_deref(), false, |items| {
                        apply_delete(
                            items,
                            &params,
                            predicate_text.as_deref(),
                            condition.as_ref(),
                        )
                    });
                    Ok((Response::new(Value::Null, affected, None), vec![]))
                }
            })
            .await
    }
}

fn apply_update(
    items: &mut [Row],
    params: &Row,
    predicate_text: Option<&str>,
    condition: Option<&datasources::Expression>,
    where_names: &[String],
) -> i64 {
    let mut count = 0i64;
    for item in items.iter_mut() {
        if !row_matches(item, params, predicate_text, condition) {
            continue;
        }
        if condition.is_some() {
            for (key, value) in params {
                if !where_names.contains(key) && !key.ends_with("__where") {
                    item.insert(key.clone(), value.clone());
                }
            }
        } else {
            let mut obj = params.clone();
            obj.remove("$where");
            item.extend(obj);
        }
        count += 1;
    }
    count
}

fn apply_delete(
    items: &mut Vec<Row>,
    params: &Row,
    predicate_text: Option<&str>,
    condition: Option<&datasources::Expression>,
) -> i64 {
    let before = items.len();
    items.retain(|row| !row_matches(row, params, predicate_text, condition));
    (before - items.len()) as i64
}

fn row_matches(
    row: &Row,
    params: &Row,
    predicate_text: Option<&str>,
    condition: Option<&datasources::Expression>,
) -> bool {
    if let Some(condition) = condition {
        return ExpressionEvaluator.evaluate(condition, row) == Value::Bool(true);
    }
    let filter = match predicate_text {
        Some(text) if text.trim_start().starts_with('{') => {
            parse_filter(Some(text)).unwrap_or_else(|_| Row::new())
        }
        _ => params.clone(),
    };
    matches(row, &filter)
}

fn table_of(statement: &dyn Statement) -> Option<String> {
    if let Some(select) = statement
        .as_any()
        .downcast_ref::<datasources::SelectStatement>()
    {
        if !select.table.is_empty() {
            return Some(select.table.clone());
        }
    }
    table_in(&datasources::DefaultStatementTextRenderer::instance().render(statement))
}

fn table_in(text: &str) -> Option<String> {
    let sql = text.trim_start();
    let lower = sql.to_ascii_lowercase();
    let skip = if lower.starts_with("insert into ") {
        "insert into ".len()
    } else if lower.starts_with("delete from ") {
        "delete from ".len()
    } else if lower.starts_with("update ") {
        "update ".len()
    } else if let Some(at) = lower.find(" from ") {
        at + " from ".len()
    } else {
        return None;
    };
    let rest = sql.get(skip..)?;
    let end = rest.find([' ', '(']).unwrap_or(rest.len());
    let name = rest[..end].trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn inserted_id(row: &Row, connection: &MemoryConnection) -> Value {
    if let Some(id @ Value::Int(_)) = row.get("id") {
        return id.clone();
    }
    Value::Int(connection.collection(|items| {
        items
            .iter()
            .filter_map(|item| match item.get("id") {
                Some(Value::Int(id)) => Some(*id),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + 1
    }))
}

fn filter_text(statement: &dyn Statement) -> String {
    if let Some(where_clause) = statement.predicate() {
        let text = datasources::DefaultStatementTextRenderer::instance().render_where(where_clause);
        if text.trim_start().starts_with('{') {
            return text;
        }
    }
    let text = datasources::DefaultStatementTextRenderer::instance().render(statement);
    if text.trim_start().starts_with('{') || text.trim().is_empty() {
        text
    } else {
        String::new()
    }
}
