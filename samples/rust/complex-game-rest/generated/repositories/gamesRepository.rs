use datasources::{Connection, CRUDRepository, CRUDRepositoryBuilder, CRUDRepositoryOptions, DictionaryMapper, Enrichment, EnrichmentPropertyType, Association, EnrichmentVia, ICRUDRepository, IReadonlyRepository, IRepository, IWhere, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Schema, SchemaColumn, Statement, Result, async_trait, CommandMiddleware, IdKey, Response, Stream};
use super::super::Game;
use super::gameSettingsRepository::GameSettingsRepository;
use super::categoriesRepository::CategoriesRepository;
use super::gamesCategoriesRepository::GamesCategoriesRepository;
use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

pub struct GamesRepository {
    pub(crate) inner: Arc<CRUDRepository<Game>>,
    pub game_settings: GameSettingsRepository,
    pub categories: CategoriesRepository,
    pub games_categories: GamesCategoriesRepository,
}

impl GamesRepository {
    pub fn new(connection: Arc<dyn Connection>) -> Result<Self> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Game".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Game".into(), "games_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("location".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()];
        options.create_properties = Some(vec!["name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options.update_properties = Some(vec!["uuid".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Game".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Game".into(), "games_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("location".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let game_settings = GameSettingsRepository::new(Arc::clone(&connection))?;
        let categories = CategoriesRepository::new(Arc::clone(&connection))?;
        let games_categories = GamesCategoriesRepository::new(Arc::clone(&connection))?;
        let mut repo = CRUDRepository::<Game>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "game_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &game_settings, None);
        repo.add_linked(Enrichment::new("categories", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("game_id", "category_id")), &categories, Arc::clone(&games_categories.inner) as Arc<dyn ICRUDRepository<_>>, None);
        Ok(Self { inner: Arc::new(repo), game_settings, categories, games_categories })
    }

    pub fn create_readonly(connection: Arc<dyn Connection>) -> Result<ReadonlyRepository<Game>> {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "Game".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Game".into(), "games_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("location".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()];
        let game_settings = GameSettingsRepository::new(Arc::clone(&connection))?;
        let categories = CategoriesRepository::new(Arc::clone(&connection))?;
        let mut repo = ReadonlyRepository::<Game>::new(connection, options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "game_id", Some("value".into()), EnrichmentPropertyType::Dictionary), &game_settings, None);
        repo.add_enriched(Enrichment::new("categories", "id", "id", Some("name".into()), EnrichmentPropertyType::List), &categories, None);
        Ok(repo)
    }
}

impl Deref for GamesRepository {
    type Target = CRUDRepository<Game>;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl EnrichmentVia<Game> for &GamesRepository {
    fn into_repos(
        self,
    ) -> (
        Arc<dyn IReadonlyRepository<Game>>,
        Option<Arc<dyn ICRUDRepository<Game>>>,
    ) {
        (
            Arc::clone(&self.inner) as Arc<dyn IReadonlyRepository<Game>>,
            Some(Arc::clone(&self.inner) as Arc<dyn ICRUDRepository<Game>>),
        )
    }
}

impl IRepository for GamesRepository {
    fn entity_type(&self) -> TypeId {
        IRepository::entity_type(self.inner.as_ref())
    }
}

#[async_trait]
impl IReadonlyRepository<Game> for GamesRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository<Game>>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Game>> {
        self.inner.as_ref().read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Game>> + Send + 'a>> {
        IReadonlyRepository::read(self.inner.as_ref(), query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Game>> {
        self.inner.as_ref().read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Game>> + Send + 'a>> {
        IReadonlyRepository::read_where(self.inner.as_ref(), clause, options)
    }
}

#[async_trait]
impl ICRUDRepository<Game> for GamesRepository {
    async fn create(&self, item: Game) -> Result<Game> {
        self.inner.create(item).await
    }

    async fn update(&self, item: Game, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::update(self.inner.as_ref(), item, by_where).await
    }

    async fn delete(&self, item: Game, by_where: Option<&dyn IWhere>) -> Result<Response> {
        ICRUDRepository::delete(self.inner.as_ref(), item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Game>> {
        self.inner.read_single_by_id(id).await
    }

    fn id_names(&self) -> Vec<String> {
        self.inner.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.inner.add_middleware(middleware)
    }
}

pub fn games_row_repository(connection: Arc<dyn Connection>) -> Result<CRUDRepository<datasources::Row>> {
    let options = CRUDRepositoryBuilder::new()
    .add({
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Game".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Game".into(), "games_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("location".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()];
        options.create_properties = Some(vec!["name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options.update_properties = Some(vec!["uuid".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options
    })
    .add_for("sqlite", {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Game".into();
        options.entity_mapper = Some(Arc::new(DictionaryMapper::new([
            ("Game".into(), "games_bases".into()),
        ])));
        options.schema = Schema::new(HashMap::from([
            ("id".into(), SchemaColumn { type_name: "biginteger".into(), nullable: false, has_default: false, is_id: true }),
            ("uuid".into(), SchemaColumn { type_name: "uuid".into(), nullable: false, has_default: true, is_id: false }),
            ("created".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("updated".into(), SchemaColumn { type_name: "datetime".into(), nullable: false, has_default: true, is_id: false }),
            ("version".into(), SchemaColumn { type_name: "binary".into(), nullable: false, has_default: true, is_id: false }),
            ("name".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("description".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("location".into(), SchemaColumn { type_name: "string".into(), nullable: false, has_default: false, is_id: false }),
            ("is_enabled".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
            ("is_deleted".into(), SchemaColumn { type_name: "boolean".into(), nullable: false, has_default: true, is_id: false }),
        ]));
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "uuid".into(), "created".into(), "updated".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()];
        options.create_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options.update_properties = Some(vec!["uuid".into(), "created".into(), "version".into(), "name".into(), "description".into(), "location".into(), "is_enabled".into(), "is_deleted".into()]);
        options
    })
    .for_connection(connection.as_ref())?;

        let game_settings = Arc::new(super::gameSettingsRepository::game_settings_row_repository(Arc::clone(&connection))?);
        let categories = Arc::new(super::categoriesRepository::categories_row_repository(Arc::clone(&connection))?);
        let games_categories = Arc::new(super::gamesCategoriesRepository::games_categories_row_repository(Arc::clone(&connection))?);
        let mut repo = CRUDRepository::<datasources::Row>::new(Arc::clone(&connection), options)?;
        repo.add_enriched(Enrichment::new("settings", "id", "game_id", Some("value".into()), EnrichmentPropertyType::Dictionary), Arc::clone(&game_settings), None);
        repo.add_linked(Enrichment::new("categories", "id", "id", Some("name".into()), EnrichmentPropertyType::List).linked(Association::new("game_id", "category_id")), Arc::clone(&categories), Arc::clone(&games_categories) as Arc<dyn ICRUDRepository<_>>, None);
        Ok(repo)
}
