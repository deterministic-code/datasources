use crate::datasource_error::DatasourceError;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::{where_of, ReadInput};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statements::{Operator, ExpressionEvaluator, Where};
use crate::value::Value;
use std::sync::{Arc, Mutex};

pub struct SharedItems {
    pub items: Mutex<Vec<Row>>,
    next_id: Mutex<i64>,
}

impl SharedItems {
    pub fn new(items: Vec<Row>) -> Arc<Self> {
        let next_id = items
            .iter()
            .filter_map(|row| match row.get("id") {
                Some(Value::Int(id)) => Some(*id),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + 1;
        Arc::new(Self {
            items: Mutex::new(items),
            next_id: Mutex::new(next_id),
        })
    }

    pub fn filter(&self, where_clause: Option<&dyn IWhere>) -> Vec<Row> {
        let items = self.items.lock().expect("items");
        match where_clause {
            None => items.clone(),
            Some(predicate) => {
                let evaluator = ExpressionEvaluator;
                items
                    .iter()
                    .filter(|row| {
                        predicate
                            .as_any()
                            .downcast_ref::<Where>()
                            .map(|statement| {
                                evaluator.evaluate(&statement.condition, row) == Value::Bool(true)
                            })
                            .unwrap_or(true)
                    })
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn filter_input(&self, query: Option<&dyn ReadInput>) -> Vec<Row> {
        self.filter(where_of(query))
    }

    pub fn insert(&self, item: &mut Row) -> Row {
        if !matches!(item.get("id"), Some(Value::Int(id)) if *id != 0) {
            let mut next = self.next_id.lock().expect("items");
            let id = *next;
            *next += 1;
            item.insert("id".into(), Value::Int(id));
        }
        self.items.lock().expect("items").push(item.clone());
        item.clone()
    }

    pub fn save(&self, item: &Row) -> Response {
        let id = item.get("id").cloned();
        let mut items = self.items.lock().expect("items");
        if let Some(index) = items.iter().position(|row| row.get("id") == id.as_ref()) {
            items[index] = item.clone();
        }
        Response::new(Value::Null, 1, None)
    }

    pub fn remove(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Response {
        let mut items = self.items.lock().expect("items");
        if let Some(predicate) = by_where {
            let evaluator = ExpressionEvaluator;
            items.retain(|row| {
                !predicate
                    .as_any()
                    .downcast_ref::<Where>()
                    .map(|statement| {
                        evaluator.evaluate(&statement.condition, row) == Value::Bool(true)
                    })
                    .unwrap_or(false)
            });
        } else {
            let id = item.get("id").cloned();
            items.retain(|row| row.get("id") != id.as_ref());
        }
        Response::new(Value::Null, 1, None)
    }

    pub fn by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        let clause = id_clause(id)?;
        Ok(self.filter(Some(&clause)).into_iter().next())
    }
}

fn id_clause(id: &dyn IdKey) -> Result<Where> {
    match (id.as_value(), id.as_row()) {
        (Some(value), _) => Ok(Where::new(Operator::equals("id", value.clone()))),
        (_, Some(row)) => row
            .iter()
            .map(|(key, value)| Operator::equals(key.clone(), value.clone()))
            .reduce(|left, right| Operator::and(left, right))
            .map(Where::new)
            .ok_or_else(|| DatasourceError::message("ReadSingleById requires an Id column")),
        _ => Err(DatasourceError::message(
            "ReadSingleById requires an Id column",
        )),
    }
}
