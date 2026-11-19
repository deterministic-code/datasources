use datasources::*;
use datasources_memory::MemoryDriver;
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

