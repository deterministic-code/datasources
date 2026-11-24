use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use async_trait::async_trait;
use datasources::*;
use futures_util::TryStreamExt;
use std::any::TypeId;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn setting_schema() -> Schema {
    let id = SchemaColumn {
        is_id: true,
        ..SchemaColumn::new("integer")
    };
    Schema::new(
        [
            ("Id", id),
            ("ParentId", SchemaColumn::new("integer")),
            ("Name", SchemaColumn::new("string")),
            ("Value", SchemaColumn::new("string")),
        ]
        .into_iter()
        .map(|(name, column)| (name.to_string(), column))
        .collect(),
    )
}

fn mapped_crud(connection: Arc<dyn Connection>, entity: &str, schema: Schema) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.schema = schema;
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

struct Counting<T: Entity> {
    inner: Arc<dyn IReadonlyRepository<T>>,
    reads: Arc<Mutex<usize>>,
}

impl<T: Entity> IRepository for Counting<T> {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl<T: Entity> IReadonlyRepository<T> for Counting<T> {
    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read(query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_where(clause, options)
    }
}

struct CountingCrud<T: Entity> {
    inner: Arc<dyn ICRUDRepository<T>>,
    reads: Arc<Mutex<usize>>,
}

impl<T: Entity> IRepository for CountingCrud<T> {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }

    fn schema(&self) -> &Schema {
        self.inner.schema()
    }
}

#[async_trait]
impl<T: Entity> IReadonlyRepository<T> for CountingCrud<T> {
    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single(query, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read(query, options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single_where(clause, options).await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_where(clause, options)
    }
}

#[async_trait]
impl<T: Entity> ICRUDRepository<T> for CountingCrud<T> {
    async fn create(&self, item: T) -> Result<T> {
        self.inner.create(item).await
    }

    async fn update(&self, item: T, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(&self, item: T, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<T>> {
        *self.reads.lock().unwrap() += 1;
        self.inner.read_single_by_id(id).await
    }
}

async fn open() -> Arc<SqliteConnection> {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    for sql in [
        "CREATE TABLE games (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        "CREATE TABLE settings (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL)",
        "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        "CREATE TABLE game_categories (id INTEGER PRIMARY KEY AUTOINCREMENT, game_id INTEGER NOT NULL, category_id INTEGER NOT NULL)",
    ] {
        command.execute_native(&Native::new(sql), None).await.unwrap();
    }
    connection
}

fn id_of(row: &Row) -> Value {
    row.get("Id").unwrap().clone()
}

struct Counters {
    games: Arc<Mutex<usize>>,
    settings: Arc<Mutex<usize>>,
    categories: Arc<Mutex<usize>>,
    links: Arc<Mutex<usize>>,
}

impl Counters {
    fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(0)),
            settings: Arc::new(Mutex::new(0)),
            categories: Arc::new(Mutex::new(0)),
            links: Arc::new(Mutex::new(0)),
        }
    }

    fn snapshot(&self) -> (usize, usize, usize, usize) {
        (
            *self.games.lock().unwrap(),
            *self.settings.lock().unwrap(),
            *self.categories.lock().unwrap(),
            *self.links.lock().unwrap(),
        )
    }
}

async fn seed(connection: Arc<dyn Connection>) -> Value {
    let games = mapped_crud(connection.clone(), "games", Schema::default());
    let settings = mapped_crud(connection.clone(), "settings", setting_schema());
    let categories = mapped_crud(connection.clone(), "categories", Schema::default());
    let links = mapped_crud(connection, "game_categories", Schema::default());
    let keep = categories
        .create(record(&[("Name", Value::from("arcade"))]))
        .await
        .unwrap();
    let mut first = None;
    for i in 0..121 {
        let game = games
            .create(record(&[("Name", Value::from(format!("g{i}")))]))
            .await
            .unwrap();
        if first.is_none() {
            first = Some(id_of(&game));
        }
        for (name, value) in [("a", "1"), ("b", "2"), ("c", "3")] {
            settings
                .create(record(&[
                    ("ParentId", id_of(&game)),
                    ("Name", Value::from(name)),
                    ("Value", Value::from(value)),
                ]))
                .await
                .unwrap();
        }
        links
            .create(record(&[
                ("GameId", id_of(&game)),
                ("CategoryId", id_of(&keep)),
            ]))
            .await
            .unwrap();
    }
    first.unwrap()
}

fn count_read<T: Entity>(inner: Arc<dyn IReadonlyRepository<T>>, reads: Arc<Mutex<usize>>) -> Arc<Counting<T>> {
    Arc::new(Counting { inner, reads })
}

fn wired(
    connection: Arc<dyn Connection>,
    counters: &Counters,
    settings: bool,
    categories: bool,
) -> CRUDRepository {
    let mut games = mapped_crud(connection.clone(), "games", Schema::default());
    if settings {
        games.add_enriched(
            Enrichment::new(
                "Settings",
                "Id",
                "ParentId",
                Some("Value".into()),
                EnrichmentPropertyType::Dictionary,
            )
            .with_dictionary_key("Name"),
            count_read(
                Arc::new(mapped_crud(connection.clone(), "settings", setting_schema())),
                counters.settings.clone(),
            ),
            None,
        );
    }
    if categories {
        let tags: Arc<dyn ICRUDRepository<Row>> =
            Arc::new(mapped_crud(connection.clone(), "categories", Schema::default()));
        let links: Arc<dyn ICRUDRepository<Row>> =
            Arc::new(mapped_crud(connection, "game_categories", Schema::default()));
        games.add_linked(
            Enrichment::new("Categories", "Id", "Id", Some("Name".into()), EnrichmentPropertyType::List)
                .linked(Association::new("GameId", "CategoryId")),
            count_read(tags as Arc<dyn IReadonlyRepository<Row>>, counters.categories.clone()),
            Arc::new(CountingCrud {
                inner: links,
                reads: counters.links.clone(),
            }),
            None,
        );
    }
    games
}

fn counted_games(games: CRUDRepository, counters: &Counters) -> CountingCrud<Row> {
    CountingCrud {
        inner: Arc::new(games),
        reads: counters.games.clone(),
    }
}

#[tokio::test]
async fn one_hundred_twenty_one_games_read_and_read_single_by_id_are_constant_queries() {
    let connection = open().await;
    let first = seed(connection.clone()).await;
    let connection: Arc<dyn Connection> = connection;

    let none = Counters::new();
    let none_repo = counted_games(wired(connection.clone(), &none, false, false), &none);
    let started = Instant::now();
    let bare: Vec<Row> = none_repo.read(None, None).try_collect().await.unwrap();
    let none_list_ms = started.elapsed();
    assert_eq!(bare.len(), 121);
    assert_eq!(none.snapshot(), (1, 0, 0, 0));

    let settings_only = Counters::new();
    let settings_repo = counted_games(wired(connection.clone(), &settings_only, true, false), &settings_only);
    let started = Instant::now();
    let with_settings: Vec<Row> = settings_repo.read(None, None).try_collect().await.unwrap();
    let settings_list_ms = started.elapsed();
    assert_eq!(with_settings.len(), 121);
    assert_eq!(with_settings[0].payload("Settings").len(), 3);
    assert_eq!(settings_only.snapshot(), (1, 1, 0, 0));

    let full = Counters::new();
    let full_repo = counted_games(wired(connection.clone(), &full, true, true), &full);
    let started = Instant::now();
    let games: Vec<Row> = full_repo.read(None, None).try_collect().await.unwrap();
    let full_list_ms = started.elapsed();
    assert_eq!(games.len(), 121);
    assert_eq!(games[0].payload("Settings").len(), 3);
    assert_eq!(full.snapshot(), (1, 1, 1, 1));

    let settings_only = Counters::new();
    let settings_only_repo = counted_games(wired(connection.clone(), &settings_only, true, true), &settings_only);
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.enrich = Some(EnrichmentSelection::only(["Settings"]));
    let listed: Vec<Row> = settings_only_repo
        .read(None, Some(&options))
        .try_collect()
        .await
        .unwrap();
    assert_eq!(listed.len(), 121);
    assert_eq!(listed[0].payload("Settings").len(), 3);
    assert_eq!(settings_only.snapshot(), (1, 1, 0, 0));

    let none_fills = Counters::new();
    let none_fills_repo = counted_games(wired(connection.clone(), &none_fills, true, true), &none_fills);
    let mut none_options = ReadonlyRepositoryMappingOptions::default();
    none_options.enrich = Some(EnrichmentSelection::none());
    let bare_fills: Vec<Row> = none_fills_repo
        .read(None, Some(&none_options))
        .try_collect()
        .await
        .unwrap();
    assert_eq!(bare_fills.len(), 121);
    assert_eq!(none_fills.snapshot(), (1, 0, 0, 0));

    let by_id = Counters::new();
    let by_id_repo = counted_games(wired(connection.clone(), &by_id, true, true), &by_id);
    let one_settings = by_id_repo
        .read_single_where(&Where::new(Operator::equals("Id", first.clone())), Some(&options))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(one_settings.payload("Settings").len(), 3);
    assert_eq!(by_id.snapshot(), (1, 1, 0, 0));

    let single = Counters::new();
    let single_repo = counted_games(wired(connection, &single, true, true), &single);
    let started = Instant::now();
    let one = single_repo.read_single_by_id(&first).await.unwrap().unwrap();
    let single_ms = started.elapsed();
    assert_eq!(one.payload("Settings").len(), 3);
    assert_eq!(single.snapshot(), (1, 1, 1, 1));

    eprintln!(
        "sqlite Game view: list none={none_list_ms:?} queries=1; list settings={settings_list_ms:?} queries=games1+settings1; list settings+categories={full_list_ms:?} queries=4; read_single_by_id={single_ms:?} queries=4 (games+settings+categories+links)"
    );
    if settings_list_ms.saturating_sub(none_list_ms) < none_list_ms / 10 {
        eprintln!(
            "dictionary settings on the 121-row list is not a large share of wall time next to the games table read"
        );
    }
    eprintln!(
        "read_single_by_id still runs four sequential queries; dictionary N+1 is not the list cost, and on a single game it is one extra settings round trip"
    );
}
