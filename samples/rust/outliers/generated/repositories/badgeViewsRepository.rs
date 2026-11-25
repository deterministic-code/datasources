use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::BadgeView;
use super::badgeAliasesRepository::BadgeAliasesRepository;
use super::badgeSettingsRepository::BadgeSettingsRepository;
use super::badgeNodesRepository::BadgeNodesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct BadgeViewsRepository {
    pub(crate) inner: Arc<CRUDRepository<BadgeView>>,
    pub badge_alias: BadgeAliasesRepository,
    pub badge_setting: BadgeSettingsRepository,
    pub badge_node: BadgeNodesRepository,
}

impl BadgeViewsRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "BadgeView".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("BadgeView".into(), "badges".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("badge_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: false, is_id: true }),
        ("code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: true }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["badge_id".into(), "code".into()];
    options.read_properties = vec!["badge_id".into(), "code".into(), "name".into()];
    options.create_properties = Some(vec!["badge_id".into(), "code".into(), "name".into()]);
    options.update_properties = Some(vec!["badge_id".into(), "code".into(), "name".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let badge_alias = BadgeAliasesRepository::new(Arc::clone(&connection))?;
        let badge_setting = BadgeSettingsRepository::new(Arc::clone(&connection))?;
        let badge_node = BadgeNodesRepository::new(Arc::clone(&connection))?;
        let options = Self::options()?;
        let mut repo = CRUDRepository::<BadgeView>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("aliases", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_alias, None);
        repo.add_enriched(Enrichment::new("settings", "badge_id", "parent_badge_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_setting, None);
        repo.add_enriched(Enrichment::new("nodes", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_node, None);
        Ok(Self { inner: Arc::new(repo), badge_alias, badge_setting, badge_node })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<BadgeView>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "BadgeView".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("BadgeView".into(), "badges".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("badge_id".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: false, is_id: true }),
            ("code".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["badge_id".into(), "code".into(), "name".into()];
        let badge_alias = BadgeAliasesRepository::new(Arc::clone(&connection))?;
        let badge_setting = BadgeSettingsRepository::new(Arc::clone(&connection))?;
        let badge_node = BadgeNodesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<BadgeView>::new(connection, options.clone())?;
        repo.add_enriched(Enrichment::new("aliases", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_alias, None);
        repo.add_enriched(Enrichment::new("settings", "badge_id", "parent_badge_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_setting, None);
        repo.add_enriched(Enrichment::new("nodes", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), &badge_node, None);
        Ok(repo)
    }
}

impl Deref for BadgeViewsRepository {
    type Target = CRUDRepository<BadgeView>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<BadgeView> for &BadgeViewsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<BadgeView>>,
        Option<Arc<dyn ICRUDRepository<BadgeView>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<BadgeView>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<BadgeView>>),
        )
    }
}

impl IRepository for BadgeViewsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<BadgeView> for BadgeViewsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<BadgeView>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<BadgeView>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<BadgeView>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<BadgeView>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<BadgeView>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<BadgeView> for BadgeViewsRepository {
    async fn create(&self, item: BadgeView) -> Result<BadgeView> {
        self.inner.create(item).await
    }

    async fn update(&self, item: BadgeView, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: BadgeView, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<BadgeView>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn badge_views_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let badge_alias = Arc::new(super::badgeAliasesRepository::badge_aliases_row_repository(Arc::clone(&connection))?);
        let badge_setting = Arc::new(super::badgeSettingsRepository::badge_settings_row_repository(Arc::clone(&connection))?);
        let badge_node = Arc::new(super::badgeNodesRepository::badge_nodes_row_repository(Arc::clone(&connection))?);
        let options = BadgeViewsRepository::options()?;
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("aliases", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), Arc::clone(&badge_alias), None);
        repo.add_enriched(Enrichment::new("settings", "badge_id", "parent_badge_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), Arc::clone(&badge_setting), None);
        repo.add_enriched(Enrichment::new("nodes", "badge_id", "parent_badge_id", None, EnrichmentPropertyType::List).with_keys(&[("badge_id", "parent_badge_id"), ("code", "parent_code")]), Arc::clone(&badge_node), None);
        Ok(repo)
}
