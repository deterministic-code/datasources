use crate::i_repository::IRepository;
use crate::query::Query;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::statements::{StatementEvaluator, WhereClause};
use crate::value::Value;
use crate::where_trait::Where;
use async_trait::async_trait;

pub struct ItemsRepository {
    items: Vec<Row>,
}

impl ItemsRepository {
    pub fn new(items: Vec<Row>) -> Self {
        Self { items }
    }

    fn filter(&self, where_clause: Option<&dyn Where>) -> Vec<Row> {
        match where_clause {
            None => self.items.clone(),
            Some(predicate) => {
                let evaluator = StatementEvaluator;
                self.items
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

impl IRepository for ItemsRepository {}

#[async_trait]
impl IReadonlyRepository for ItemsRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        Ok(self
            .filter(query.and_then(Query::where_clause))
            .into_iter()
            .next())
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        Ok(self.filter(query.and_then(Query::where_clause)))
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        Ok(self.filter(Some(where_clause)).into_iter().next())
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        Ok(self.filter(Some(where_clause)))
    }
}
