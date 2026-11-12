use crate::filter::{matches, parse_filter};
use datasources::{
    parameters_to_map, DeleteStatement, Expression, ExpressionEvaluator, InsertStatement,
    Operator, Parameter, Response, Result, Row, SelectStatement, Statement, StatementParser,
    UpdateStatement, Value, Where, WhereText, DatasourceError, DefaultStatementTextRenderer, IWhere,
};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
#[cfg_attr(not(test), allow(dead_code))]
pub enum MemoryData {
    Rows(Vec<Row>),
    Collections(HashMap<String, Vec<Row>>),
}

pub struct MemoryEngine {
    collections: Mutex<HashMap<String, Vec<Row>>>,
    collection: Mutex<String>,
    inserted: Mutex<HashMap<String, Value>>,
    snapshot: Mutex<Option<HashMap<String, Vec<Row>>>>,
    in_transaction: Mutex<bool>,
    records: Mutex<Vec<Row>>,
}

impl MemoryEngine {
    pub fn new(collection: impl Into<String>) -> Self {
        Self {
            collections: Mutex::new(HashMap::new()),
            collection: Mutex::new(collection.into()),
            inserted: Mutex::new(HashMap::new()),
            snapshot: Mutex::new(None),
            in_transaction: Mutex::new(false),
            records: Mutex::new(Vec::new()),
        }
    }

    pub fn has_collection(&self, name: &str) -> bool {
        self.collections.lock().unwrap().contains_key(name)
    }

    pub fn collection<R>(&self, name: &str, f: impl FnOnce(&mut Vec<Row>) -> R) -> R {
        let mut map = self.collections.lock().unwrap();
        f(map.entry(name.to_string()).or_default())
    }

    pub fn seed(&self, data: &MemoryData, collection: &str) {
        match data {
            MemoryData::Rows(rows) if !self.has_collection(collection) => {
                self.collections
                    .lock()
                    .unwrap()
                    .insert(collection.to_string(), rows.clone());
            }
            MemoryData::Collections(named) => {
                let mut map = self.collections.lock().unwrap();
                for (name, rows) in named {
                    map.entry(name.clone()).or_insert_with(|| rows.clone());
                }
            }
            _ => {}
        }
    }

    pub fn collection_name(&self) -> String {
        self.collection.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn set_collection(&self, name: impl Into<String>) {
        *self.collection.lock().unwrap() = name.into();
    }

    pub fn in_transaction(&self) -> bool {
        *self.in_transaction.lock().unwrap()
    }

    pub fn records(&self) -> Vec<Row> {
        self.records.lock().unwrap().clone()
    }

    pub fn begin(&self) {
        *self.snapshot.lock().unwrap() = Some(self.collections.lock().unwrap().clone());
        *self.in_transaction.lock().unwrap() = true;
    }

    pub fn commit(&self) {
        *self.in_transaction.lock().unwrap() = false;
        *self.snapshot.lock().unwrap() = None;
    }

    pub fn abort(&self) {
        if let Some(snapshot) = self.snapshot.lock().unwrap().take() {
            *self.collections.lock().unwrap() = snapshot;
        }
        *self.in_transaction.lock().unwrap() = false;
    }

    pub fn execute(&self, text: &str, parameters: Option<&[Parameter]>) -> Result<Response> {
        *self.records.lock().unwrap() = Vec::new();
        if let Some((op, where_text)) = named_op(text) {
            return self.named(&op, parameters, where_text);
        }
        if text.trim_start().starts_with('{') || text.trim().is_empty() {
            let mut filter = parse_filter(Some(text))?;
            filter.extend(parameters_to_map(parameters));
            if let Some(Value::Text(table)) = filter.get("__inserted").cloned() {
                let column = match filter.get("__column") {
                    Some(Value::Text(name)) => name.clone(),
                    _ => "id".into(),
                };
                let rows = self.inserted_row(&table, &column);
                let count = rows.len() as i64;
                *self.records.lock().unwrap() = rows;
                return Ok(Response::new(Value::Null, count, None));
            }
            let collection = match filter.remove("__table") {
                Some(Value::Text(name)) => name,
                _ => self.collection_name(),
            };
            let rows = self.collection(&collection, |items| {
                items
                    .iter()
                    .filter(|row| matches(row, &filter))
                    .cloned()
                    .collect::<Vec<_>>()
            });
            let count = rows.len() as i64;
            *self.records.lock().unwrap() = rows;
            return Ok(Response::new(Value::Null, count, None));
        }
        let (sql, json_where) = json_where_split(text);
        let extra: Option<&dyn IWhere> = json_where.as_ref().map(|where_clause| where_clause as &dyn IWhere);
        let statement = StatementParser::parse(sql)
            .map_err(|_| DatasourceError::message(format!("Unsupported Memory command: {text}")))?;
        if let Some(insert) = statement.as_any().downcast_ref::<InsertStatement>() {
            return self.insert(insert, parameters);
        }
        if let Some(update) = statement.as_any().downcast_ref::<UpdateStatement>() {
            return self.change(update.table(), parameters, extra.or(update.predicate()));
        }
        if let Some(delete) = statement.as_any().downcast_ref::<DeleteStatement>() {
            return self.remove(delete.table(), parameters, extra.or(delete.predicate()));
        }
        if let Some(select) = statement.as_any().downcast_ref::<SelectStatement>() {
            return self.read_select(select, parameters);
        }
        Err(DatasourceError::message(format!(
            "Unsupported Memory command: {text}"
        )))
    }

    fn named(
        &self,
        op: &str,
        parameters: Option<&[Parameter]>,
        where_text: Option<String>,
    ) -> Result<Response> {
        let obj = parameters_to_map(parameters);
        match op {
            "insert" => self.insert_row(obj, &self.collection_name()),
            "update" => {
                let mut next = obj;
                let from_param = next.remove("$where").and_then(|value| match value {
                    Value::Text(text) => Some(text),
                    _ => None,
                });
                let filter = if let Some(text) = where_text.or(from_param) {
                    parse_filter(Some(&text))?
                } else if let Some(id) = next.get("id").cloned() {
                    let mut filter = Row::new();
                    filter.insert("id".into(), id);
                    filter
                } else {
                    next.clone()
                };
                Ok(self.apply_update(&self.collection_name(), next, filter))
            }
            _ => {
                let filter = if let Some(text) = where_text {
                    parse_filter(Some(&text))?
                } else {
                    obj
                };
                Ok(self.apply_delete(&self.collection_name(), filter))
            }
        }
    }

    fn insert(&self, insert: &InsertStatement, parameters: Option<&[Parameter]>) -> Result<Response> {
        let table = if insert.table().is_empty() {
            self.collection_name()
        } else {
            insert.table().to_string()
        };
        self.insert_row(parameters_to_map(parameters), &table)
    }

    fn insert_row(&self, mut obj: Row, table: &str) -> Result<Response> {
        let id = self.next_id(&obj, table);
        obj.insert("id".into(), id.clone());
        self.collection(table, |items| items.push(obj.clone()));
        self.inserted
            .lock()
            .unwrap()
            .insert(table.to_string(), id.clone());
        Ok(Response::new(id, 1, None))
    }

    fn change(
        &self,
        table: &str,
        parameters: Option<&[Parameter]>,
        where_clause: Option<&dyn IWhere>,
    ) -> Result<Response> {
        let obj = parameters_to_map(parameters);
        let key = self.table_key(table, false);
        if let Some(clause) = where_clause.and_then(|w| w.as_any().downcast_ref::<Where>()) {
            let names = set_names(parameters, clause);
            let affected = self.collection(&key, |items| {
                let mut count = 0i64;
                for item in items.iter_mut() {
                    if !bound_match(clause, item, parameters) {
                        continue;
                    }
                    for name in &names {
                        if let Some(value) = obj.get(name) {
                            item.insert(name.clone(), value.clone());
                        }
                    }
                    count += 1;
                }
                count
            });
            return Ok(Response::new(Value::Null, affected, None));
        }
        let mut next = obj;
        next.remove("$where");
        let rendered = where_clause.map(|w| DefaultStatementTextRenderer::instance().render_where(w));
        let filter = if rendered.as_deref().is_some_and(|text| text.trim_start().starts_with('{')) {
            parse_filter(rendered.as_deref())?
        } else if let Some(id) = next.get("id").cloned() {
            let mut filter = Row::new();
            filter.insert("id".into(), id);
            filter
        } else {
            next.clone()
        };
        Ok(self.apply_update(&key, next, filter))
    }

    fn remove(
        &self,
        table: &str,
        parameters: Option<&[Parameter]>,
        where_clause: Option<&dyn IWhere>,
    ) -> Result<Response> {
        let obj = parameters_to_map(parameters);
        let key = self.table_key(table, false);
        if let Some(clause) = where_clause.and_then(|w| w.as_any().downcast_ref::<Where>()) {
            let affected = self.collection(&key, |items| {
                let before = items.len();
                items.retain(|item| !bound_match(clause, item, parameters));
                (before - items.len()) as i64
            });
            return Ok(Response::new(Value::Null, affected, None));
        }
        let rendered = where_clause.map(|w| DefaultStatementTextRenderer::instance().render_where(w));
        let filter = if rendered.as_deref().is_some_and(|text| text.trim_start().starts_with('{')) {
            parse_filter(rendered.as_deref())?
        } else {
            obj
        };
        Ok(self.apply_delete(&key, filter))
    }

    fn read_select(&self, select: &SelectStatement, parameters: Option<&[Parameter]>) -> Result<Response> {
        let key = self.table_key(&select.table, false);
        let rows = if let Some(where_) = select.predicate().and_then(|p| p.as_any().downcast_ref::<Where>()) {
            self.collection(&key, |items| {
                items
                    .iter()
                    .filter(|row| bound_match(where_, row, parameters))
                    .cloned()
                    .collect::<Vec<_>>()
            })
        } else {
            self.collection(&key, |items| items.clone())
        };
        let count = rows.len() as i64;
        *self.records.lock().unwrap() = rows;
        Ok(Response::new(Value::Null, count, None))
    }

    fn table_key(&self, table: &str, create: bool) -> String {
        if !table.is_empty() && table != "mem" && (create || self.has_collection(table)) {
            table.to_string()
        } else {
            self.collection_name()
        }
    }

    fn inserted_row(&self, table: &str, column: &str) -> Vec<Row> {
        match self.inserted.lock().unwrap().get(table) {
            Some(Value::Null) | None => vec![],
            Some(id) => {
                let mut row = Row::new();
                row.insert(column.to_string(), id.clone());
                vec![row]
            }
        }
    }

    fn apply_update(&self, table: &str, obj: Row, filter: Row) -> Response {
        let affected = self.collection(table, |items| {
            let mut count = 0i64;
            for item in items.iter_mut() {
                if matches(item, &filter) {
                    item.extend(obj.clone());
                    count += 1;
                }
            }
            count
        });
        Response::new(Value::Null, affected, None)
    }

    fn apply_delete(&self, table: &str, filter: Row) -> Response {
        let affected = self.collection(table, |items| {
            let before = items.len();
            items.retain(|row| !matches(row, &filter));
            (before - items.len()) as i64
        });
        Response::new(Value::Null, affected, None)
    }

    fn next_id(&self, row: &Row, table: &str) -> Value {
        if let Some(Value::Int(id)) = row.get("id") {
            return Value::Int(*id);
        }
        let max = self.collection(table, |items| {
            items
                .iter()
                .filter_map(|item| match item.get("id") {
                    Some(Value::Int(id)) => Some(*id),
                    _ => None,
                })
                .max()
                .unwrap_or(0)
        });
        Value::Int(max + 1)
    }
}

fn bound_match(where_clause: &Where, row: &Row, parameters: Option<&[Parameter]>) -> bool {
    let bound = parameters_to_map(parameters);
    match_expr(&where_clause.condition, row, &bound, parameters)
}

fn match_expr(
    expression: &Expression,
    row: &Row,
    bound: &Row,
    parameters: Option<&[Parameter]>,
) -> bool {
    match expression {
        Expression::Operator(Operator::And { left, right }) => {
            match_expr(left, row, bound, parameters) && match_expr(right, row, bound, parameters)
        }
        Expression::Operator(Operator::Or { left, right }) => {
            match_expr(left, row, bound, parameters) || match_expr(right, row, bound, parameters)
        }
        Expression::Operator(Operator::Equals { left, right: _ }) => {
            if let Expression::Identifier(identifier) = left.as_ref() {
                row.get(&identifier.name)
                    .is_some_and(|cell| cell == bound.get(&identifier.name).unwrap_or(&Value::Null))
            } else {
                ExpressionEvaluator.evaluate(expression, &context(row, parameters)) == Value::Bool(true)
            }
        }
        Expression::Operator(Operator::In { left, right }) => in_match(left, right, row, bound),
        Expression::Operator(Operator::NotIn { left, right }) => !in_match(left, right, row, bound),
        _ => ExpressionEvaluator.evaluate(expression, &context(row, parameters)) == Value::Bool(true),
    }
}

fn in_match(left: &Expression, right: &Expression, row: &Row, bound: &Row) -> bool {
    let Expression::Identifier(identifier) = left else {
        return false;
    };
    let Expression::Array(arr) = right else {
        return false;
    };
    let Some(left_val) = row.get(&identifier.name) else {
        return false;
    };
    arr.elements.iter().enumerate().any(|(i, element)| {
        let value = match element {
            Expression::Literal(literal) if literal.value != Value::Null => &literal.value,
            _ => bound
                .get(&format!("{}_{i}", identifier.name))
                .unwrap_or(&Value::Null),
        };
        left_val == value
    })
}

fn json_where_split(text: &str) -> (&str, Option<WhereText>) {
    let lower = text.to_ascii_lowercase();
    let Some(mark) = lower.rfind(" where ") else {
        return (text, None);
    };
    let tail = text[mark + 7..].trim();
    if !tail.starts_with('{') {
        return (text, None);
    }
    (text[..mark].trim_end(), Some(WhereText::new(tail)))
}

fn named_op(text: &str) -> Option<(String, Option<String>)> {
    let trimmed = text.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower == "insert" || lower == "update" || lower == "delete" {
        return Some((lower, None));
    }
    for op in ["update", "delete"] {
        let prefix = format!("{op} ");
        if let Some(rest) = lower.strip_prefix(&prefix) {
            let where_text = trimmed[prefix.len()..].trim();
            if rest.trim_start().starts_with('{') {
                return Some((op.to_string(), Some(where_text.to_string())));
            }
        }
    }
    None
}

fn context(row: &Row, parameters: Option<&[Parameter]>) -> Row {
    let mut ctx = parameters_to_map(parameters);
    ctx.extend(row.clone());
    ctx
}

fn set_names(parameters: Option<&[Parameter]>, where_clause: &Where) -> Vec<String> {
    let bound: std::collections::HashSet<_> = where_clause
        .parameters()
        .into_iter()
        .map(|parameter| parameter.name)
        .collect();
    parameters
        .unwrap_or(&[])
        .iter()
        .map(|parameter| parameter.name.clone())
        .filter(|name| !bound.contains(name) && !name.ends_with("__where"))
        .collect()
}
