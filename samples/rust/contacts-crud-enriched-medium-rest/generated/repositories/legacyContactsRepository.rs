use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::LegacyContact;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct LegacyContactsRepository {
    pub(crate) inner: Arc<CRUDRepository<LegacyContact>>,
}

impl LegacyContactsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "legacy_contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec![];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let repo = CRUDRepository::<LegacyContact>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<LegacyContact>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "legacy_contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        let repo = ReadonlyRepository::<LegacyContact>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for LegacyContactsRepository {
    type Target = CRUDRepository<LegacyContact>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<LegacyContact> for &LegacyContactsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<LegacyContact>>,
        Option<Arc<dyn ICRUDRepository<LegacyContact>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<LegacyContact>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<LegacyContact>>),
        )
    }
}

impl IRepository for LegacyContactsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<LegacyContact> for LegacyContactsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<LegacyContact>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<LegacyContact>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<LegacyContact>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<LegacyContact>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<LegacyContact>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<LegacyContact> for LegacyContactsRepository {
    async fn create(&self, item: LegacyContact) -> Result<LegacyContact> {
        self.inner.create(item).await
    }

    async fn update(&self, item: LegacyContact, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: LegacyContact, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<LegacyContact>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn legacy_contacts_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "legacy_contacts".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec![];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("mysql", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .add_for("postgres", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "LegacyContact".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("LegacyContact".into(), "OldContactsTbl".into()),
        ])));
        options.property_mappings = Some(Arc::new(DictionaryMapper::new([
            ("key".into(), "CntID".into()),
            ("first_name".into(), "FirstNm".into()),
            ("last_name".into(), "LastNm".into()),
            ("email".into(), "EmailAddr".into()),
            ("imported_at".into(), "ImpDate".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("key".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("first_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("last_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("email".into(), SchemaColumn { type_name: "string".into(), nullable: true, has_default: false, is_id: false }),
            ("imported_at".into(), SchemaColumn { type_name: "datetime".into(), nullable: true, has_default: false, is_id: false }),
        ]));
        options.ids = vec!["key".into()];
        options.read_properties = vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()];
        options.create_properties = Some(vec!["key".into(), "first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options.update_properties = Some(vec!["first_name".into(), "last_name".into(), "email".into(), "imported_at".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

    CRUDRepository::<datasources::Row>::new(connection, options)
}
