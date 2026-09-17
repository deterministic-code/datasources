use crate::query::Query;
use crate::repository::ReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
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
            Some(predicate) => self
                .items
                .iter()
                .filter(|row| super::where_match::row_matches(row, predicate))
                .cloned()
                .collect(),
        }
    }
}

#[async_trait]
impl ReadonlyRepository for ItemsRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        Ok(self
            .filter(query.and_then(Query::where_clause))
            .into_iter()
            .next())
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        Ok(self.filter(query.and_then(Query::where_clause)))
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        Ok(self.filter(Some(where_clause)).into_iter().next())
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        Ok(self.filter(Some(where_clause)))
    }
}
