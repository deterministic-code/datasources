use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::read_input::{where_of, ReadInput};
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::statements::{StatementEvaluator, Where};
use crate::value::Value;
use async_trait::async_trait;

pub struct ItemsRepository {
    items: Vec<Row>,
}

impl ItemsRepository {
    pub fn new(items: Vec<Row>) -> Self {
        Self { items }
    }

    fn filter(&self, where_clause: Option<&dyn IWhere>) -> Vec<Row> {
        match where_clause {
            None => self.items.clone(),
            Some(predicate) => {
                let evaluator = StatementEvaluator;
                self.items
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
}

impl IRepository for ItemsRepository {}

#[async_trait]
impl IReadonlyRepository for ItemsRepository {
    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        Ok(self.filter(where_of(query)).into_iter().next())
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        Ok(self.filter(where_of(query)))
    }
}
