use crate::enriched_readonly_repository::EnrichedReadonlyRepository;
use crate::query::Query;
use crate::repository::ReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ContactsService {
    inner: EnrichedReadonlyRepository,
}

impl ContactsService {
    pub fn new(
        contacts: Arc<dyn ReadonlyRepository>,
        types: Arc<dyn ReadonlyRepository>,
        sources: Arc<dyn ReadonlyRepository>,
        addresses: Arc<dyn ReadonlyRepository>,
        phones: Arc<dyn ReadonlyRepository>,
    ) -> Self {
        let mut inner = EnrichedReadonlyRepository::new(contacts);
        inner
            .add_enriched(
                "contact_type_name",
                "contact_type_id",
                types,
                "id",
                Some("name".into()),
                false,
            )
            .add_enriched(
                "contact_source_name",
                "contact_source_id",
                sources,
                "id",
                Some("name".into()),
                false,
            )
            .add_enriched("addresses", "id", addresses, "contact_id", None, true)
            .add_enriched("phones", "id", phones, "contact_id", None, true);
        Self { inner }
    }
}

#[async_trait]
impl ReadonlyRepository for ContactsService {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.inner.get_single(query).await
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.inner.get(query).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.inner.get_single_where(where_clause).await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.inner.get_where(where_clause).await
    }
}
