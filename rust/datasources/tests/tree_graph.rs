use datasources::*;
use datasources_memory::MemoryDriver;
use futures_util::TryStreamExt;
use std::sync::Arc;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

async fn setup(rows: &mut [Row]) -> Arc<dyn Connection> {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let command = connection.create_command();
    for row in rows.iter_mut() {
        let params: Vec<Parameter> = row
            .iter()
            .map(|(name, value)| Parameter::new(name.clone(), value.clone()))
            .collect();
        let response = command.execute("insert", Some(&params)).await.unwrap();
        row.insert("id".into(), response.result);
    }
    connection
}

fn mapped(connection: Arc<dyn Connection>) -> ReadonlyRepository {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    ReadonlyRepository::<Row>::new(connection, options).unwrap()
}

fn text(row: &Row, key: &str) -> String {
    match row.get(key) {
        Some(Value::Text(value)) => value.clone(),
        _ => panic!("{key}"),
    }
}

fn flatten(node: &Row) -> Vec<String> {
    let mut names = vec![text(node, "Name")];
    for child in node.payload("Children") {
        names.extend(flatten(&child));
    }
    names
}

#[tokio::test]
async fn tree_fills_three_levels_and_stops_on_a_cycle() {
    let mut rows = [
        record(&[("name", Value::from("food"))]),
        record(&[("name", Value::from("fruit"))]),
        record(&[("name", Value::from("apple"))]),
    ];
    let connection = setup(&mut rows).await;
    let food_id = rows[0].get("id").unwrap().clone();
    let fruit_id = rows[1].get("id").unwrap().clone();
    let apple_id = rows[2].get("id").unwrap().clone();
    let command = connection.create_command();
    command
        .execute(
            "update",
            Some(&[
                Parameter::new("id", fruit_id.clone()),
                Parameter::new("parent_category_id", food_id.clone()),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "update",
            Some(&[
                Parameter::new("id", apple_id.clone()),
                Parameter::new("parent_category_id", fruit_id),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "update",
            Some(&[
                Parameter::new("id", food_id.clone()),
                Parameter::new("parent_category_id", apple_id),
            ]),
        )
        .await
        .unwrap();
    let via = Arc::new(mapped(Arc::clone(&connection)));
    let mut repo = mapped(connection);
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        via,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let food = repo
        .read_single_where(&Where::new(Operator::equals("Id", food_id)), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(text(&food, "Name"), "food");
    let children = food.payload("Children");
    assert_eq!(children.len(), 1);
    assert_eq!(text(&children[0], "Name"), "fruit");
    let apples = children[0].payload("Children");
    assert_eq!(text(&apples[0], "Name"), "apple");
    assert!(apples[0].payload("Children").is_empty());
}

#[tokio::test]
async fn tree_matches_a_composite_parent() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "insert",
            Some(&[
                Parameter::new("region_id", Value::from(1i64)),
                Parameter::new("category_id", Value::from(10i64)),
                Parameter::new("name", Value::from("food")),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "insert",
            Some(&[
                Parameter::new("region_id", Value::from(1i64)),
                Parameter::new("category_id", Value::from(11i64)),
                Parameter::new("parent_region_id", Value::from(1i64)),
                Parameter::new("parent_category_id", Value::from(10i64)),
                Parameter::new("name", Value::from("fruit")),
            ]),
        )
        .await
        .unwrap();
    let via = Arc::new(mapped(Arc::clone(&connection)));
    let mut repo = mapped(connection);
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "RegionId",
            "ParentRegionId",
            None,
            EnrichmentPropertyType::List,
        )
        .with_keys(&[("RegionId", "ParentRegionId"), ("CategoryId", "ParentCategoryId")]),
        via,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let food = repo
        .read_single_where(&Where::new(Operator::equals("CategoryId", Value::from(10i64))), None)
        .await
        .unwrap()
        .unwrap();
    let children = food.payload("Children");
    assert_eq!(text(&children[0], "Name"), "fruit");
    assert!(children[0].payload("Children").is_empty());
}

#[tokio::test]
async fn tree_read_returns_each_row_once_under_the_roots() {
    let mut rows = [
        record(&[("name", Value::from("food"))]),
        record(&[("name", Value::from("fruit"))]),
        record(&[("name", Value::from("apple"))]),
    ];
    let connection = setup(&mut rows).await;
    let food_id = rows[0].get("id").unwrap().clone();
    let fruit_id = rows[1].get("id").unwrap().clone();
    let apple_id = rows[2].get("id").unwrap().clone();
    let command = connection.create_command();
    command
        .execute(
            "update",
            Some(&[
                Parameter::new("id", fruit_id.clone()),
                Parameter::new("parent_category_id", food_id),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "update",
            Some(&[
                Parameter::new("id", apple_id),
                Parameter::new("parent_category_id", fruit_id.clone()),
            ]),
        )
        .await
        .unwrap();
    let via = Arc::new(mapped(Arc::clone(&connection)));
    let mut repo = mapped(connection);
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        via,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let roots: Vec<_> = repo.read(None, None).try_collect().await.unwrap();
    assert_eq!(roots.iter().map(|row| text(row, "Name")).collect::<Vec<_>>(), ["food"]);
    assert_eq!(flatten(&roots[0]), ["food", "fruit", "apple"]);
    let fruit = repo
        .read_single_where(&Where::new(Operator::equals("Id", fruit_id)), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(text(&fruit, "Name"), "fruit");
    let apples = fruit.payload("Children");
    assert_eq!(text(&apples[0], "Name"), "apple");
}

#[tokio::test]
async fn tree_read_returns_the_composite_root() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "insert",
            Some(&[
                Parameter::new("region_id", Value::from(1i64)),
                Parameter::new("category_id", Value::from(10i64)),
                Parameter::new("name", Value::from("food")),
            ]),
        )
        .await
        .unwrap();
    command
        .execute(
            "insert",
            Some(&[
                Parameter::new("region_id", Value::from(1i64)),
                Parameter::new("category_id", Value::from(11i64)),
                Parameter::new("parent_region_id", Value::from(1i64)),
                Parameter::new("parent_category_id", Value::from(10i64)),
                Parameter::new("name", Value::from("fruit")),
            ]),
        )
        .await
        .unwrap();
    let via = Arc::new(mapped(Arc::clone(&connection)));
    let mut repo = mapped(connection);
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "RegionId",
            "ParentRegionId",
            None,
            EnrichmentPropertyType::List,
        )
        .with_keys(&[("RegionId", "ParentRegionId"), ("CategoryId", "ParentCategoryId")]),
        via,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let roots: Vec<_> = repo.read(None, None).try_collect().await.unwrap();
    assert_eq!(roots.iter().map(|row| text(row, "Name")).collect::<Vec<_>>(), ["food"]);
    let children = roots[0].payload("Children");
    assert_eq!(text(&children[0], "Name"), "fruit");
}

#[tokio::test]
async fn graph_fills_edges_copies_a_diamond_and_drops_a_self_loop() {
    let mut nodes = [
        record(&[("value", Value::from("a"))]),
        record(&[("value", Value::from("b"))]),
        record(&[("value", Value::from("c"))]),
        record(&[("value", Value::from("d"))]),
    ];
    let connection = setup(&mut nodes).await;
    let a = nodes[0].get("id").unwrap().clone();
    let b = nodes[1].get("id").unwrap().clone();
    let c = nodes[2].get("id").unwrap().clone();
    let d = nodes[3].get("id").unwrap().clone();
    let mut links = [
        record(&[("from_node_id", a.clone()), ("to_node_id", b.clone())]),
        record(&[("from_node_id", a.clone()), ("to_node_id", c.clone())]),
        record(&[("from_node_id", b), ("to_node_id", d.clone())]),
        record(&[("from_node_id", c), ("to_node_id", d.clone())]),
        record(&[("from_node_id", d), ("to_node_id", a.clone())]),
    ];
    let edges = setup(&mut links).await;
    let via = Arc::new(mapped(Arc::clone(&connection)));
    let link = Arc::new(mapped(edges));
    let mut repo = mapped(connection);
    repo.add_enriched(
        Enrichment::new("Edges", "Id", "Id", None, EnrichmentPropertyType::List)
            .linked(Association::new("FromNodeId", "ToNodeId")),
        via,
        Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(link))),
    );
    let loaded = repo
        .read_single_where(&Where::new(Operator::equals("Id", a)), None)
        .await
        .unwrap()
        .unwrap();
    let edges = loaded.payload("Edges");
    assert_eq!(edges.len(), 2);
    let mut names: Vec<_> = edges.iter().map(|row| text(row, "Value")).collect();
    names.sort();
    assert_eq!(names, ["b", "c"]);
}

fn mapped_crud(connection: Arc<dyn Connection>) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

fn mapped_write(
    connection: Arc<dyn Connection>,
    create: Arc<dyn CreateEnrichmentStrategy>,
) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.write = Some(Arc::new(DefaultWriteEnrichmentStrategy::new(
        None,
        None,
        Some(create),
        false,
    )));
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

#[tokio::test]
async fn tree_create_writes_parents_first_by_name() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let via = Arc::new(mapped_crud(Arc::clone(&connection)));
    let mut repo = mapped_write(
        connection,
        Arc::new(TreeCreateEnrichmentStrategy::new()),
    );
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::clone(&via) as Arc<dyn ICRUDRepository>,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let apple = record(&[("Name", Value::from("apple"))]);
    let fruit = record(&[
        ("Name", Value::from("fruit")),
        ("Children", encode_rows(&[apple])),
    ]);
    repo.create(record(&[
        ("Name", Value::from("food")),
        ("Children", encode_rows(&[fruit])),
    ]))
    .await
    .unwrap();
    let food = repo
        .read_single_where(&Where::new(Operator::equals("Name", Value::from("food"))), None)
        .await
        .unwrap()
        .unwrap();
    let children = food.payload("Children");
    assert_eq!(text(&children[0], "Name"), "fruit");
    assert_eq!(text(&children[0].payload("Children")[0], "Name"), "apple");
}

#[tokio::test]
async fn tree_create_throws_on_a_cycle() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let via = Arc::new(mapped_crud(Arc::clone(&connection)));
    let mut repo = mapped_write(
        Arc::clone(&connection),
        Arc::new(TreeCreateEnrichmentStrategy::new()),
    );
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::clone(&via) as Arc<dyn ICRUDRepository>,
        None,
    );
    let food = record(&[("Name", Value::from("food"))]);
    let apple = record(&[
        ("Name", Value::from("apple")),
        ("Children", encode_rows(&[food.clone()])),
    ]);
    let fruit = record(&[
        ("Name", Value::from("fruit")),
        ("Children", encode_rows(&[apple])),
    ]);
    let error = repo
        .create(record(&[
            ("Name", Value::from("food")),
            ("Children", encode_rows(&[fruit])),
        ]))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("food"), "{error}");
}

#[tokio::test]
async fn tree_create_stamps_composite_parents() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut via_options = CRUDRepositoryOptions::default();
    via_options.ids = vec!["RegionId".into(), "CategoryId".into()];
    via_options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    let via = Arc::new(CRUDRepository::<Row>::new(Arc::clone(&connection), via_options).unwrap());
    let mut options = CRUDRepositoryOptions::default();
    options.ids = vec!["RegionId".into(), "CategoryId".into()];
    options.property_mappings = Some(Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.write = Some(Arc::new(DefaultWriteEnrichmentStrategy::new(
        None,
        None,
        Some(Arc::new(TreeCreateEnrichmentStrategy::new())),
        false,
    )));
    let mut repo = CRUDRepository::<Row>::new(connection, options).unwrap();
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "RegionId",
            "ParentRegionId",
            None,
            EnrichmentPropertyType::List,
        )
        .with_keys(&[("RegionId", "ParentRegionId"), ("CategoryId", "ParentCategoryId")]),
        Arc::clone(&via) as Arc<dyn ICRUDRepository>,
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    repo.create(record(&[
        ("RegionId", Value::from(1i64)),
        ("CategoryId", Value::from(1i64)),
        ("Name", Value::from("west")),
        (
            "Children",
            encode_rows(&[record(&[
                ("RegionId", Value::from(1i64)),
                ("CategoryId", Value::from(2i64)),
                ("Name", Value::from("coast")),
            ])]),
        ),
    ]))
    .await
    .unwrap();
    let west = repo
        .read_single_where(&Where::new(Operator::equals("Name", Value::from("west"))), None)
        .await
        .unwrap()
        .unwrap();
    let child = &west.payload("Children")[0];
    assert_eq!(text(child, "Name"), "coast");
    assert_eq!(child.get("ParentRegionId"), Some(&Value::from(1i64)));
    assert_eq!(child.get("ParentCategoryId"), Some(&Value::from(1i64)));
}

#[tokio::test]
async fn graph_create_writes_nodes_then_edges_and_skips_a_diamond() {
    let nodes = MemoryDriver.connection(None);
    nodes.open().await.unwrap();
    let edges = MemoryDriver.connection(None);
    edges.open().await.unwrap();
    let via = Arc::new(mapped_crud(Arc::clone(&nodes)));
    let link: Arc<dyn ICRUDRepository> = Arc::new(mapped_crud(edges));
    let mut repo = mapped_write(
        nodes,
        Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&link))),
    );
    repo.add_enriched(
        Enrichment::new("Edges", "Id", "Id", None, EnrichmentPropertyType::List)
            .linked(Association::new("FromNodeId", "ToNodeId")),
        Arc::clone(&via) as Arc<dyn ICRUDRepository>,
        Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(Arc::clone(&link)))),
    );
    let d = record(&[("Value", Value::from("d"))]);
    repo.create(record(&[
        ("Value", Value::from("a")),
        (
            "Edges",
            encode_rows(&[
                record(&[
                    ("Value", Value::from("b")),
                    ("Edges", encode_rows(&[d.clone()])),
                ]),
                record(&[
                    ("Value", Value::from("c")),
                    ("Edges", encode_rows(&[d])),
                ]),
            ]),
        ),
    ]))
    .await
    .unwrap();
    let node_rows: Vec<_> = via.read(None, None).try_collect().await.unwrap();
    let edge_rows: Vec<_> = link.read(None, None).try_collect().await.unwrap();
    assert_eq!(node_rows.len(), 4);
    assert_eq!(edge_rows.len(), 4);
}

#[tokio::test]
async fn graph_create_writes_a_loop() {
    let nodes = MemoryDriver.connection(None);
    nodes.open().await.unwrap();
    let edges = MemoryDriver.connection(None);
    edges.open().await.unwrap();
    let via = Arc::new(mapped_crud(Arc::clone(&nodes)));
    let link: Arc<dyn ICRUDRepository> = Arc::new(mapped_crud(edges));
    let mut repo = mapped_write(
        nodes,
        Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&link))),
    );
    repo.add_enriched(
        Enrichment::new("Edges", "Id", "Id", None, EnrichmentPropertyType::List)
            .linked(Association::new("FromNodeId", "ToNodeId")),
        Arc::clone(&via) as Arc<dyn ICRUDRepository>,
        Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(Arc::clone(&link)))),
    );
    let mut a = record(&[("Value", Value::from("a"))]);
    let b = record(&[
        ("Value", Value::from("b")),
        ("Edges", encode_rows(&[a.clone()])),
    ]);
    a.insert("Edges".into(), encode_rows(&[b]));
    repo.create(a).await.unwrap();
    let node_rows: Vec<_> = via.read(None, None).try_collect().await.unwrap();
    let edge_rows: Vec<_> = link.read(None, None).try_collect().await.unwrap();
    assert_eq!(node_rows.len(), 2);
    assert_eq!(edge_rows.len(), 2);
    let loaded_a = repo
        .read_single_where(&Where::new(Operator::equals("Value", Value::from("a"))), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(text(&loaded_a.payload("Edges")[0], "Value"), "b");
    let loaded_b = repo
        .read_single_where(&Where::new(Operator::equals("Value", Value::from("b"))), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(text(&loaded_b.payload("Edges")[0], "Value"), "a");
}

