use async_trait::async_trait;
use datasources::*;
use datasources_memory::MemoryDriver;
use futures_core::Stream;
use futures_util::TryStreamExt;
use std::any::TypeId;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

async fn setup() -> Arc<dyn Connection> {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    connection
}

fn mapped_crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    mapped_crud_schema(connection, entity, Schema::default())
}

fn mapped_crud_schema(
    connection: Arc<dyn Connection>,
    entity: &str,
    schema: Schema,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(
        Casing::Pascal,
        Casing::Snake,
    )));
    options.schema = schema;
    CRUDRepository::<Row>::new(connection, options).unwrap()
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

fn object_fill() -> Enrichment {
    Enrichment::new(
        "KindName",
        "Kind",
        "Id",
        Some("Name".into()),
        EnrichmentPropertyType::Object,
    )
}

fn list_fill() -> Enrichment {
    Enrichment::new("Children", "Id", "ParentId", None, EnrichmentPropertyType::List)
}

fn dictionary_fill() -> Enrichment {
    Enrichment::new(
        "Settings",
        "Id",
        "ParentId",
        Some("Value".into()),
        EnrichmentPropertyType::Dictionary,
    )
    .with_dictionary_key("Name")
}

fn linked_fill() -> Enrichment {
    Enrichment::new("Tags", "Id", "Id", None, EnrichmentPropertyType::List)
        .linked(Association::new("ParentId", "TagId"))
}

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(value)) => value.clone(),
        _ => String::new(),
    }
}

fn id_of(row: &Row) -> Value {
    row.get("Id").unwrap().clone()
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

    async fn update(
        &self,
        item: T,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        self.inner.update(item, by_where).await
    }

    async fn delete(
        &self,
        item: T,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        self.inner.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<T>> {
        self.inner.read_single_by_id(id).await
    }
}

fn as_read<R, T>(repo: Arc<R>) -> Arc<dyn IReadonlyRepository<T>>
where
    R: IReadonlyRepository<T> + 'static,
    T: Entity,
{
    repo
}

fn count<T: Entity>(inner: Arc<dyn IReadonlyRepository<T>>, reads: Arc<Mutex<usize>>) -> Arc<Counting<T>> {
    Arc::new(Counting { inner, reads })
}

async fn read_with(
    connection: Arc<dyn Connection>,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy<Row, Row>>,
    fill: Enrichment,
    via_entity: &str,
) -> Vec<Row> {
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let via = Arc::new(mapped_crud(connection, via_entity));
    parents.add_enriched(fill, via, Some(strategy));
    parents.read(None, None).try_collect().await.unwrap()
}

async fn read_settings(
    connection: Arc<dyn Connection>,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy<Row, Row>>,
) -> Vec<Row> {
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let via = Arc::new(mapped_crud_schema(connection, "Setting", setting_schema()));
    parents.add_enriched(dictionary_fill(), via, Some(strategy));
    parents.read(None, None).try_collect().await.unwrap()
}

async fn read_tags(
    connection: Arc<dyn Connection>,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy<Row, Row>>,
) -> Vec<Row> {
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let tags: Arc<dyn ICRUDRepository<Row>> = Arc::new(mapped_crud(connection.clone(), "Tag"));
    let links: Arc<dyn ICRUDRepository<Row>> = Arc::new(mapped_crud(connection, "ParentTag"));
    parents.add_linked(linked_fill(), tags, links, Some(strategy));
    parents.read(None, None).try_collect().await.unwrap()
}

#[tokio::test]
async fn parity_object_missing_duplicate_and_null_keys() {
    let connection = setup().await;
    let kinds = mapped_crud(connection.clone(), "Kind");
    let parents = mapped_crud(connection.clone(), "Parent");
    let person = kinds
        .create(record(&[("Name", Value::from("person"))]))
        .await
        .unwrap();
    kinds
        .create(record(&[("Name", Value::from("gone"))]))
        .await
        .unwrap();
    let kept = parents
        .create(record(&[("Kind", id_of(&person))]))
        .await
        .unwrap();
    let missing = parents
        .create(record(&[("Kind", Value::Int(99))]))
        .await
        .unwrap();
    let duplicate = parents
        .create(record(&[("Kind", id_of(&person))]))
        .await
        .unwrap();
    parents.create(record(&[])).await.unwrap();
    let expected = read_with(
        connection.clone(),
        Arc::new(DefaultReadonlyEnrichmentStrategy),
        object_fill(),
        "Kind",
    )
    .await;
    let actual = read_with(
        connection,
        Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new()),
        object_fill(),
        "Kind",
    )
    .await;
    let names = |rows: &[Row]| rows.iter().map(|row| text(row, "KindName")).collect::<Vec<_>>();
    assert_eq!(names(&actual), names(&expected));
    assert_eq!(
        text(actual.iter().find(|row| row.get("Id") == kept.get("Id")).unwrap(), "KindName"),
        "person"
    );
    assert_eq!(
        text(actual.iter().find(|row| row.get("Id") == missing.get("Id")).unwrap(), "KindName"),
        ""
    );
    assert_eq!(
        text(
            actual.iter().find(|row| row.get("Id") == duplicate.get("Id")).unwrap(),
            "KindName"
        ),
        "person"
    );
}

#[tokio::test]
async fn parity_list_missing_duplicate_and_null_keys() {
    let connection = setup().await;
    let parents = mapped_crud(connection.clone(), "Parent");
    let children = mapped_crud(connection.clone(), "Child");
    let shared = parents.create(record(&[])).await.unwrap();
    let lonely = parents.create(record(&[])).await.unwrap();
    parents.create(record(&[])).await.unwrap();
    children
        .create(record(&[("ParentId", id_of(&shared)), ("Label", Value::from("a"))]))
        .await
        .unwrap();
    children
        .create(record(&[("ParentId", id_of(&shared)), ("Label", Value::from("b"))]))
        .await
        .unwrap();
    children
        .create(record(&[("ParentId", id_of(&lonely)), ("Label", Value::from("x"))]))
        .await
        .unwrap();
    let expected = read_with(
        connection.clone(),
        Arc::new(DefaultReadonlyEnrichmentStrategy),
        list_fill(),
        "Child",
    )
    .await;
    let actual = read_with(
        connection,
        Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new()),
        list_fill(),
        "Child",
    )
    .await;
    let labels = |rows: &[Row]| {
        rows.iter()
            .map(|row| {
                row.related::<Row>("Children")
                    .into_iter()
                    .map(|child| text(&child, "Label"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(labels(&actual), labels(&expected));
}

#[tokio::test]
async fn parity_dictionary_missing_duplicate_and_null_keys() {
    let connection = setup().await;
    let parents = mapped_crud(connection.clone(), "Parent");
    let settings = mapped_crud_schema(connection.clone(), "Setting", setting_schema());
    let shared = parents.create(record(&[])).await.unwrap();
    let empty = parents.create(record(&[])).await.unwrap();
    parents.create(record(&[])).await.unwrap();
    settings
        .create(record(&[
            ("ParentId", id_of(&shared)),
            ("Name", Value::from("theme")),
            ("Value", Value::from("dark")),
        ]))
        .await
        .unwrap();
    settings
        .create(record(&[
            ("ParentId", id_of(&shared)),
            ("Name", Value::from("density")),
            ("Value", Value::from("comfortable")),
        ]))
        .await
        .unwrap();
    let expected = read_settings(connection.clone(), Arc::new(DefaultReadonlyEnrichmentStrategy)).await;
    let actual = read_settings(
        connection,
        Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new()),
    )
    .await;
    assert_eq!(
        actual.iter().find(|row| row.get("Id") == shared.get("Id")).unwrap().payload("Settings").len(),
        expected.iter().find(|row| row.get("Id") == shared.get("Id")).unwrap().payload("Settings").len()
    );
    assert_eq!(
        actual.iter().find(|row| row.get("Id") == empty.get("Id")).unwrap().payload("Settings").len(),
        0
    );
}

#[tokio::test]
async fn parity_linked_missing_duplicate_and_null_keys() {
    let connection = setup().await;
    let parents = mapped_crud(connection.clone(), "Parent");
    let tags = mapped_crud(connection.clone(), "Tag");
    let links = mapped_crud(connection.clone(), "ParentTag");
    let keep = tags.create(record(&[("Name", Value::from("keep"))])).await.unwrap();
    let drop = tags.create(record(&[("Name", Value::from("drop"))])).await.unwrap();
    let tagged = parents.create(record(&[])).await.unwrap();
    let other = parents.create(record(&[])).await.unwrap();
    parents.create(record(&[])).await.unwrap();
    links
        .create(record(&[("ParentId", id_of(&tagged)), ("TagId", id_of(&keep))]))
        .await
        .unwrap();
    links
        .create(record(&[("ParentId", id_of(&tagged)), ("TagId", id_of(&drop))]))
        .await
        .unwrap();
    links
        .create(record(&[("ParentId", id_of(&other)), ("TagId", id_of(&keep))]))
        .await
        .unwrap();
    let expected = read_tags(connection.clone(), Arc::new(DefaultReadonlyEnrichmentStrategy)).await;
    let actual = read_tags(
        connection,
        Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new()),
    )
    .await;
    let names = |rows: &[Row]| {
        rows.iter()
            .map(|row| {
                row.related::<Row>("Tags")
                    .into_iter()
                    .map(|tag| text(&tag, "Name"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(names(&actual), names(&expected));
}

#[tokio::test]
async fn one_hundred_parents_is_one_read_per_enrichment() {
    let connection = setup().await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let children = Arc::new(mapped_crud(connection, "Child"));
    for i in 0..100 {
        let parent = parents.create(record(&[])).await.unwrap();
        children
            .create(record(&[
                ("ParentId", id_of(&parent)),
                ("Label", Value::from(i.to_string())),
            ]))
            .await
            .unwrap();
    }
    let reads = Arc::new(Mutex::new(0));
    parents.add_enriched(
        list_fill(),
        count(as_read(children), reads.clone()),
        Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new())),
    );
    let items: Vec<Row> = parents.read(None, None).try_collect().await.unwrap();
    assert_eq!(items.len(), 100);
    assert_eq!(*reads.lock().unwrap(), 1);
}

#[tokio::test]
async fn zero_queries_when_parents_empty() {
    let connection = setup().await;
    let reads = Arc::new(Mutex::new(0));
    LoadLevelOrderReadonlyEnrichmentStrategy::new()
        .apply(
            count(as_read(Arc::new(mapped_crud(connection, "Child"))), reads.clone()),
            &list_fill(),
            &mut Vec::<Row>::new(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(*reads.lock().unwrap(), 0);
}

#[tokio::test]
async fn zero_queries_when_keys_null() {
    let connection = setup().await;
    let reads = Arc::new(Mutex::new(0));
    LoadLevelOrderReadonlyEnrichmentStrategy::new()
        .apply(
            count(as_read(Arc::new(mapped_crud(connection, "Kind"))), reads.clone()),
            &object_fill(),
            &mut vec![record(&[]), record(&[])],
            None,
        )
        .await
        .unwrap();
    assert_eq!(*reads.lock().unwrap(), 0);
}

#[tokio::test]
async fn chunks_ten_keys_by_three() {
    let connection = setup().await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let children = Arc::new(mapped_crud(connection, "Child"));
    for i in 0..10 {
        let parent = parents.create(record(&[])).await.unwrap();
        children
            .create(record(&[
                ("ParentId", id_of(&parent)),
                ("Label", Value::from(i.to_string())),
            ]))
            .await
            .unwrap();
    }
    let reads = Arc::new(Mutex::new(0));
    parents.add_enriched(
        list_fill(),
        count(as_read(children), reads.clone()),
        Some(Arc::new(
            LoadLevelOrderReadonlyEnrichmentStrategy::new().with_batch_size(3),
        )),
    );
    let items: Vec<Row> = parents.read(None, None).try_collect().await.unwrap();
    assert_eq!(
        items.iter().map(|row| row.related::<Row>("Children").len()).sum::<usize>(),
        10
    );
    assert_eq!(*reads.lock().unwrap(), 4);
}

#[tokio::test]
async fn nested_parent_child_grandchild_is_three_queries() {
    let connection = setup().await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let mut children = mapped_crud(connection.clone(), "Child");
    let notes = Arc::new(mapped_crud(connection, "Note"));
    let parent = parents.create(record(&[])).await.unwrap();
    let child = children
        .create(record(&[("ParentId", id_of(&parent)), ("Label", Value::from("a"))]))
        .await
        .unwrap();
    notes
        .create(record(&[("ChildId", id_of(&child)), ("Text", Value::from("n"))]))
        .await
        .unwrap();
    let reads = Arc::new(Mutex::new(0));
    children.add_enriched(
        Enrichment::new("Notes", "Id", "ChildId", None, EnrichmentPropertyType::List),
        count(as_read(notes), reads.clone()),
        Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new())),
    );
    let children = Arc::new(children);
    parents.add_enriched(
        list_fill(),
        count(as_read(children), reads.clone()),
        Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new())),
    );
    let counted = count(as_read(Arc::new(parents)), reads.clone());
    let items: Vec<Row> = counted.read(None, None).try_collect().await.unwrap();
    assert_eq!(
        text(
            &items[0].related::<Row>("Children")[0].related::<Row>("Notes")[0],
            "Text"
        ),
        "n"
    );
    assert_eq!(*reads.lock().unwrap(), 3);
}

#[tokio::test]
async fn composite_keys_match_all_pairs() {
    let connection = setup().await;
    let mut seats = mapped_crud(connection.clone(), "Seat");
    let marks = Arc::new(mapped_crud(connection, "Mark"));
    let left = seats
        .create(record(&[("PlayId", Value::Int(1)), ("UserId", Value::Int(10))]))
        .await
        .unwrap();
    let right = seats
        .create(record(&[("PlayId", Value::Int(1)), ("UserId", Value::Int(11))]))
        .await
        .unwrap();
    marks
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(10)),
            ("Note", Value::from("left")),
        ]))
        .await
        .unwrap();
    marks
        .create(record(&[
            ("PlayId", Value::Int(1)),
            ("UserId", Value::Int(11)),
            ("Note", Value::from("right")),
        ]))
        .await
        .unwrap();
    let reads = Arc::new(Mutex::new(0));
    seats.add_enriched(
        Enrichment::new("Marks", "PlayId", "PlayId", None, EnrichmentPropertyType::List)
            .with_keys(&[("PlayId", "PlayId"), ("UserId", "UserId")]),
        count(as_read(marks), reads.clone()),
        Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new())),
    );
    let items: Vec<Row> = seats.read(None, None).try_collect().await.unwrap();
    assert_eq!(
        text(
            &items
                .iter()
                .find(|row| row.get("Id") == left.get("Id"))
                .unwrap()
                .related::<Row>("Marks")[0],
            "Note"
        ),
        "left"
    );
    assert_eq!(
        text(
            &items
                .iter()
                .find(|row| row.get("Id") == right.get("Id"))
                .unwrap()
                .related::<Row>("Marks")[0],
            "Note"
        ),
        "right"
    );
    assert_eq!(*reads.lock().unwrap(), 1);
}

#[tokio::test]
async fn linked_two_queries_and_empty_when_unlinked() {
    let connection = setup().await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let tags: Arc<dyn ICRUDRepository<Row>> = Arc::new(mapped_crud(connection.clone(), "Tag"));
    let links: Arc<dyn ICRUDRepository<Row>> = Arc::new(mapped_crud(connection, "ParentTag"));
    let keep = tags.create(record(&[("Name", Value::from("keep"))])).await.unwrap();
    let tagged = parents.create(record(&[])).await.unwrap();
    let bare = parents.create(record(&[])).await.unwrap();
    links
        .create(record(&[("ParentId", id_of(&tagged)), ("TagId", id_of(&keep))]))
        .await
        .unwrap();
    let reads = Arc::new(Mutex::new(0));
    let counted_tags = count(tags.clone() as Arc<dyn IReadonlyRepository<Row>>, reads.clone());
    let counted_links = Arc::new(CountingCrud {
        inner: links,
        reads: reads.clone(),
    });
    parents.add_linked(
        linked_fill(),
        counted_tags,
        counted_links,
        Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new())),
    );
    let items: Vec<Row> = parents.read(None, None).try_collect().await.unwrap();
    assert_eq!(
        text(
            &items
                .iter()
                .find(|row| row.get("Id") == tagged.get("Id"))
                .unwrap()
                .related::<Row>("Tags")[0],
            "Name"
        ),
        "keep"
    );
    assert!(items
        .iter()
        .find(|row| row.get("Id") == bare.get("Id"))
        .unwrap()
        .related::<Row>("Tags")
        .is_empty());
    assert_eq!(*reads.lock().unwrap(), 2);
}

#[tokio::test]
async fn one_hundred_twenty_one_games_three_settings_one_attribute_is_three_queries() {
    dictionary_query_count(Some(Arc::new(LoadLevelOrderReadonlyEnrichmentStrategy::new()))).await;
}

#[tokio::test]
async fn omitted_strategy_dictionary_is_one_read_per_enrichment() {
    dictionary_query_count(None).await;
}

#[tokio::test]
async fn default_strategy_dictionary_is_one_read_per_enrichment() {
    dictionary_query_count(Some(Arc::new(DefaultReadonlyEnrichmentStrategy))).await;
}

#[tokio::test]
async fn enrich_only_settings_skips_other_fills() {
    let connection = setup().await;
    let mut games = mapped_crud(connection.clone(), "Game");
    let settings = Arc::new(mapped_crud_schema(connection.clone(), "Setting", setting_schema()));
    let attributes = Arc::new(mapped_crud_schema(connection, "GameAttribute", setting_schema()));
    for i in 0..121 {
        let game = games
            .create(record(&[("Name", Value::from(format!("g{i}")))]))
            .await
            .unwrap();
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
        attributes
            .create(record(&[
                ("ParentId", id_of(&game)),
                ("Name", Value::from("theme")),
                ("Value", Value::from("dark")),
            ]))
            .await
            .unwrap();
    }
    let settings_reads = Arc::new(Mutex::new(0));
    let attribute_reads = Arc::new(Mutex::new(0));
    games.add_enriched(dictionary_fill(), count(as_read(settings), settings_reads.clone()), None);
    games.add_enriched(
        Enrichment::new(
            "Attributes",
            "Id",
            "ParentId",
            Some("Value".into()),
            EnrichmentPropertyType::Dictionary,
        )
        .with_dictionary_key("Name"),
        count(as_read(attributes), attribute_reads.clone()),
        None,
    );
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.enrich = Some(EnrichmentSelection::only(["Settings"]));
    let items: Vec<Row> = games.read(None, Some(&options)).try_collect().await.unwrap();
    assert_eq!(items.len(), 121);
    assert_eq!(items[0].payload("Settings").len(), 3);
    assert_eq!(*settings_reads.lock().unwrap(), 1);
    assert_eq!(*attribute_reads.lock().unwrap(), 0);
}

#[tokio::test]
async fn default_zero_queries_when_parents_empty() {
    let connection = setup().await;
    let reads = Arc::new(Mutex::new(0));
    DefaultReadonlyEnrichmentStrategy
        .apply(
            count(as_read(Arc::new(mapped_crud(connection, "Child"))), reads.clone()),
            &list_fill(),
            &mut Vec::<Row>::new(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(*reads.lock().unwrap(), 0);
}

#[tokio::test]
async fn default_zero_queries_when_keys_null() {
    let connection = setup().await;
    let reads = Arc::new(Mutex::new(0));
    DefaultReadonlyEnrichmentStrategy
        .apply(
            count(as_read(Arc::new(mapped_crud(connection, "Kind"))), reads.clone()),
            &object_fill(),
            &mut vec![record(&[]), record(&[])],
            None,
        )
        .await
        .unwrap();
    assert_eq!(*reads.lock().unwrap(), 0);
}

async fn dictionary_query_count(
    strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy<Row, Row>>>,
) {
    let connection = setup().await;
    let mut games = mapped_crud(connection.clone(), "Game");
    let settings = Arc::new(mapped_crud_schema(connection.clone(), "Setting", setting_schema()));
    let attributes = Arc::new(mapped_crud_schema(
        connection,
        "GameAttribute",
        setting_schema(),
    ));
    for i in 0..121 {
        let game = games
            .create(record(&[("Name", Value::from(format!("g{i}")))]))
            .await
            .unwrap();
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
        attributes
            .create(record(&[
                ("ParentId", id_of(&game)),
                ("Name", Value::from("theme")),
                ("Value", Value::from("dark")),
            ]))
            .await
            .unwrap();
    }
    let reads = Arc::new(Mutex::new(0));
    games.add_enriched(
        dictionary_fill(),
        count(as_read(settings), reads.clone()),
        strategy.clone(),
    );
    games.add_enriched(
        Enrichment::new(
            "Attributes",
            "Id",
            "ParentId",
            Some("Value".into()),
            EnrichmentPropertyType::Dictionary,
        )
        .with_dictionary_key("Name"),
        count(as_read(attributes), reads.clone()),
        strategy,
    );
    let counted = count(as_read(Arc::new(games)), reads.clone());
    let items: Vec<Row> = counted.read(None, None).try_collect().await.unwrap();
    assert_eq!(items.len(), 121);
    assert_eq!(items[0].payload("Settings").len(), 3);
    assert_eq!(*reads.lock().unwrap(), 3);
}
