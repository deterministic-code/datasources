use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::ContactCard;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct ContactCardsRepository {
    pub(crate) inner: Arc<CRUDRepository<ContactCard>>,
}

impl ContactCardsRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactCard".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactCard".into(), "contact_cards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec![];
    options.read_properties = vec!["display_name".into()];
    options.create_properties = Some(vec!["display_name".into()]);
    options.update_properties = Some(vec!["display_name".into()]);

        let repo = CRUDRepository::<ContactCard>::new(connection, options)?;
        Ok(Self { inner: Arc::new(repo) })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<ContactCard>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "ContactCard".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("ContactCard".into(), "contact_cards".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["display_name".into()];
        let repo = ReadonlyRepository::<ContactCard>::new(connection, options)?;
        Ok(repo)
    }
}

impl Deref for ContactCardsRepository {
    type Target = CRUDRepository<ContactCard>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<ContactCard> for &ContactCardsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<ContactCard>>,
        Option<Arc<dyn ICRUDRepository<ContactCard>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<ContactCard>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<ContactCard>>),
        )
    }
}

impl IRepository for ContactCardsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<ContactCard> for ContactCardsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<ContactCard>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<ContactCard>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<ContactCard>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<ContactCard>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<ContactCard>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<ContactCard> for ContactCardsRepository {
    async fn create(&self, item: ContactCard) -> Result<ContactCard> {
        self.inner.create(item).await
    }

    async fn update(&self, item: ContactCard, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: ContactCard, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<ContactCard>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn contact_cards_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "ContactCard".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("ContactCard".into(), "contact_cards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("display_name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec![];
    options.read_properties = vec!["display_name".into()];
    options.create_properties = Some(vec!["display_name".into()]);
    options.update_properties = Some(vec!["display_name".into()]);

    CRUDRepository::<datasources::Row>::new(connection, options)
}
