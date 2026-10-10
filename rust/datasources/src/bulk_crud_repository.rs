use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_unit_of_work::IUnitOfWork;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::single_command_i_bulk_write_strategy::SingleCommandIBulkWriteStrategy;
use crate::stored_enrichment::StoredEnrichment;
use crate::unit_of_work_transaction::UnitOfWorkTransaction;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct BulkCRUDRepository {
    crud: Arc<CRUDRepository>,
    strategy: Arc<dyn BulkWriteStrategy>,
    use_transaction: bool,
}

impl BulkCRUDRepository {
    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Self::with_strategy(
            connection,
            options,
            Arc::new(SingleCommandIBulkWriteStrategy::new()),
        )
    }

    pub fn with_strategy(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
        strategy: Arc<dyn BulkWriteStrategy>,
    ) -> Result<Self> {
        let use_transaction = options
            .write
            .as_ref()
            .is_some_and(|write| write.use_transaction());
        strategy.set_options(options.clone());
        strategy.set_connection(Arc::clone(&connection));
        let crud = Arc::new(CRUDRepository::new(connection, options)?);
        strategy.set_repo(Arc::clone(&crud) as Arc<dyn ICRUDRepository>);
        strategy.set_command(crud.command());
        Ok(Self {
            crud,
            strategy,
            use_transaction,
        })
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        let use_transaction = write.as_ref().is_some_and(|write| write.use_transaction());
        let crud = Arc::new(CRUDRepository::memory_with(items, write));
        let strategy = Arc::new(SingleCommandIBulkWriteStrategy::new());
        strategy.set_repo(Arc::clone(&crud) as Arc<dyn ICRUDRepository>);
        Self {
            crud,
            strategy,
            use_transaction,
        }
    }

    pub fn add_enriched(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        let strategy = strategy.unwrap_or_else(|| Arc::new(DefaultReadonlyEnrichmentStrategy));
        let (via, persist) = via.into_repos();
        self.crud
            .enrichments
            .lock()
            .expect("enrichments")
            .push(StoredEnrichment::direct(enrichment, via, persist, strategy));
        self
    }

    async fn in_transaction<T, F, Fut>(
        transactional: bool,
        connection: Arc<dyn Connection>,
        write: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        if !transactional {
            return write().await;
        }
        let mut work = UnitOfWorkTransaction::new(connection);
        work.begin().await?;
        let result = write().await;
        match &result {
            Ok(_) => {
                if let Err(error) = work.commit().await {
                    work.abort().await?;
                    return Err(error);
                }
            }
            Err(_) => {
                work.abort().await?;
            }
        }
        result
    }

    pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Vec<Row>> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&self.crud.connection),
            || self.strategy.create(items),
        )
        .await?;
        Ok(items.to_vec())
    }

    pub async fn update_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&self.crud.connection),
            || self.strategy.update(items, by_where),
        )
        .await
    }

    pub async fn delete_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&self.crud.connection),
            || self.strategy.delete(items, by_where),
        )
        .await
    }
}

impl Deref for BulkCRUDRepository {
    type Target = CRUDRepository;

    fn deref(&self) -> &Self::Target {
        &self.crud
    }
}

impl DerefMut for BulkCRUDRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::get_mut(&mut self.crud).expect("bulk repo")
    }
}

impl IRepository for BulkCRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.crud.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for BulkCRUDRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.crud.read_single(query, parameters, options).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        self.crud.read(query, parameters, options).await
    }
}

#[async_trait]
impl ICRUDRepository for BulkCRUDRepository {
    fn id_names(&self) -> Vec<String> {
        self.crud.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.crud.add_middleware(middleware);
    }

    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.crud.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.crud.read_single_by_id(id).await
    }
}
