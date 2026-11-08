use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Result, async_trait, CommandMiddleware, IdKey, IWhere, ReadInput, Response, Stream};
use super::super::ContactChangeLog;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ContactChangeLogsRepository {
    pub(crate) inner: Arc<CRUDRepository<ContactChangeLog>>,
}

impl ContactChangeLogsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false, is_id: false }),
        ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
        ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    options.create_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    options.update_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);

        let repo = CRUDRepository::<ContactChangeLog>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactChangeLog>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "ContactChangeLog".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactChangeLog".into(), "contact_change_logs".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
            ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false, is_id: false }),
            ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
            ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
        let repo = ReadonlyRepository::<ContactChangeLog>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for ContactChangeLogsRepository {
    type Target = CRUDRepository<ContactChangeLog>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<ContactChangeLog> for &ContactChangeLogsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<ContactChangeLog>>,
        Option<Arc<dyn ICRUDRepository<ContactChangeLog>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<ContactChangeLog>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<ContactChangeLog>>),
        )
    }
}

impl IRepository for ContactChangeLogsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<ContactChangeLog> for ContactChangeLogsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<ContactChangeLog>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: &dyn ReadInput,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<ContactChangeLog>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: &'a dyn ReadInput,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<ContactChangeLog>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }
}

#[async_trait]
impl ICRUDRepository<ContactChangeLog> for ContactChangeLogsRepository {
    async fn create(&self, item: ContactChangeLog) -> Result<ContactChangeLog> {
        self.inner.create(item).await
    }

    async fn update(&self, item: ContactChangeLog, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: ContactChangeLog, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<ContactChangeLog>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn contact_change_logs_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactChangeLog".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactChangeLog".into(), "contact_change_logs".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: false }),
        ("occurred_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: false, is_id: false }),
        ("contact_id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: false }),
        ("changed_by".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ("change_summary".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()];
    options.create_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);
    options.update_properties = Some(vec!["occurred_at".into(), "contact_id".into(), "changed_by".into(), "change_summary".into()]);

    CRUDRepository::<datasources::Row>::new(connection, options)
}
