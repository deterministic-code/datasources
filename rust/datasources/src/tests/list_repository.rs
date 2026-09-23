use crate::i_repository::IRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::statements::{StatementEvaluator, WhereClause};
use crate::value::Value;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct ListRepository {
    pub items: Mutex<Vec<Row>>,
    next_id: Mutex<i64>,
}

impl ListRepository {
    pub fn new(items: Vec<Row>) -> Self {
        let next_id = items
            .iter()
            .filter_map(|row| match row.get("id") {
                Some(Value::Int(id)) => Some(*id),
                _ => None,
            })
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            items: Mutex::new(items),
            next_id: Mutex::new(next_id),
        }
    }

    pub fn snapshot(&self) -> Vec<Row> {
        self.items.lock().expect("list").clone()
    }

    fn filter(&self, where_clause: Option<&dyn Where>) -> Vec<Row> {
        let items = self.items.lock().expect("list");
        match where_clause {
            None => items.clone(),
            Some(predicate) => {
                let evaluator = StatementEvaluator;
                items
                    .iter()
                    .filter(|row| {
                        predicate
                            .as_any()
                            .downcast_ref::<WhereClause>()
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
}

impl IRepository for ListRepository {}

#[async_trait]
impl IReadonlyRepository for ListRepository {
    async fn read_single(&self, query: Option<&dyn Statement>) -> Result<Option<Row>> {
        Ok(self
            .filter(query.and_then(Statement::predicate))
            .into_iter()
            .next())
    }

    async fn read(&self, query: Option<&dyn Statement>) -> Result<Vec<Row>> {
        Ok(self.filter(query.and_then(Statement::predicate)))
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        Ok(self.filter(Some(where_clause)).into_iter().next())
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        Ok(self.filter(Some(where_clause)))
    }
}

#[async_trait]
impl ICRUDRepository for ListRepository {
    async fn create(&self, item: &mut Row) -> Result<Response> {
        let mut row = item.clone();
        let id = match row.get("id") {
            Some(Value::Int(id)) if *id != 0 => *id,
            _ => {
                let mut next = self.next_id.lock().expect("list");
                let id = *next;
                *next += 1;
                row.insert("id".into(), Value::Int(id));
                id
            }
        };
        self.items.lock().expect("list").push(row);
        Ok(Response::new(Value::Int(id), 1))
    }

    async fn update(&self, item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        let id = item.get("id").cloned();
        let mut items = self.items.lock().expect("list");
        if let Some(index) = items.iter().position(|row| row.get("id") == id.as_ref()) {
            items[index] = item.clone();
        }
        Ok(Response::new(Value::Null, 1))
    }

    async fn delete(&self, item: &Row, _by_where: Option<&dyn Where>) -> Result<Response> {
        let id = item.get("id").cloned();
        self.items
            .lock()
            .expect("list")
            .retain(|row| row.get("id") != id.as_ref());
        Ok(Response::new(Value::Null, 1))
    }
}
