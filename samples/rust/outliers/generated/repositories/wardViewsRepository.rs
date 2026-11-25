use datasources::{Connection, CRUDRepository, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::WardView;
use super::wardAliasesRepository::WardAliasesRepository;
use super::wardSettingsRepository::WardSettingsRepository;
use super::wardNodesRepository::WardNodesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct WardViewsRepository {
    pub(crate) inner: Arc<CRUDRepository<WardView>>,
    pub ward_alias: WardAliasesRepository,
    pub ward_setting: WardSettingsRepository,
    pub ward_node: WardNodesRepository,
}

impl WardViewsRepository {
    pub fn options() -> Result<CRUDRepositoryOptions> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "WardView".into();
    options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
        ("WardView".into(), "wards".into()),
    ])));
    options.schema = Schema::new(HashMap::from([
        ("region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("ward_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
        ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
    ]));
    options.ids = vec!["region_id".into(), "ward_id".into()];
    options.read_properties = vec!["region_id".into(), "ward_id".into(), "name".into()];
    options.create_properties = Some(vec!["region_id".into(), "ward_id".into(), "name".into()]);
    options.update_properties = Some(vec!["region_id".into(), "ward_id".into(), "name".into()]);

        Ok(options)
    }

    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
        let ward_alias = WardAliasesRepository::new(Arc::clone(&connection))?;
        let ward_setting = WardSettingsRepository::new(Arc::clone(&connection))?;
        let ward_node = WardNodesRepository::new(Arc::clone(&connection))?;
        let options = Self::options()?;
        let mut repo = CRUDRepository::<WardView>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("aliases", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_alias, None);
        repo.add_enriched(Enrichment::new("settings", "region_id", "parent_region_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_setting, None);
        repo.add_enriched(Enrichment::new("nodes", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_node, None);
        Ok(Self { inner: Arc::new(repo), ward_alias, ward_setting, ward_node })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<WardView>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "WardView".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("WardView".into(), "wards".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("region_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("ward_id".into(), SchemaColumn { type_name: "integer".into(), nullable: false, has_default: false, is_id: true }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
        ]));
        options.read_properties = vec!["region_id".into(), "ward_id".into(), "name".into()];
        let ward_alias = WardAliasesRepository::new(Arc::clone(&connection))?;
        let ward_setting = WardSettingsRepository::new(Arc::clone(&connection))?;
        let ward_node = WardNodesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<WardView>::new(connection, options.clone())?;
        repo.add_enriched(Enrichment::new("aliases", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_alias, None);
        repo.add_enriched(Enrichment::new("settings", "region_id", "parent_region_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_setting, None);
        repo.add_enriched(Enrichment::new("nodes", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), &ward_node, None);
        Ok(repo)
    }
}

impl Deref for WardViewsRepository {
    type Target = CRUDRepository<WardView>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<WardView> for &WardViewsRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<WardView>>,
        Option<Arc<dyn ICRUDRepository<WardView>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<WardView>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<WardView>>),
        )
    }
}

impl IRepository for WardViewsRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl IReadonlyRepository<WardView> for WardViewsRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<WardView>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<WardView>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<WardView>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<WardView>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<WardView>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<WardView> for WardViewsRepository {
    async fn create(&self, item: WardView) -> Result<WardView> {
        self.inner.create(item).await
    }

    async fn update(&self, item: WardView, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: WardView, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<WardView>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn ward_views_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
        let ward_alias = Arc::new(super::wardAliasesRepository::ward_aliases_row_repository(Arc::clone(&connection))?);
        let ward_setting = Arc::new(super::wardSettingsRepository::ward_settings_row_repository(Arc::clone(&connection))?);
        let ward_node = Arc::new(super::wardNodesRepository::ward_nodes_row_repository(Arc::clone(&connection))?);
        let options = WardViewsRepository::options()?;
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("aliases", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), Arc::clone(&ward_alias), None);
        repo.add_enriched(Enrichment::new("settings", "region_id", "parent_region_id", Some("value".into()), EnrichmentPropertyType::Dictionary).with_dictionary_key("name").with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), Arc::clone(&ward_setting), None);
        repo.add_enriched(Enrichment::new("nodes", "region_id", "parent_region_id", None, EnrichmentPropertyType::List).with_keys(&[("region_id", "parent_region_id"), ("ward_id", "parent_ward_id")]), Arc::clone(&ward_node), None);
        Ok(repo)
}
