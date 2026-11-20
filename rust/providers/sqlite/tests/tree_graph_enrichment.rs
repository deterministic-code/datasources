use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use datasources::{
    Association, Casing, ChangeCasingMapper, Command, Connection, CRUDRepository,
    CRUDRepositoryOptions, DefaultWriteEnrichmentStrategy, Entity, Enrichment, EnrichmentPropertyType,
    GraphCreateEnrichmentStrategy, GraphReadonlyEnrichmentStrategy, ICRUDRepository, Native, Operator,
    ReadonlyRepository, ReadonlyRepositoryMappingOptions, Row, TreeCreateEnrichmentStrategy,
    TreeReadonlyEnrichmentStrategy, Where, encode_rows,
};
use futures_util::TryStreamExt;
use std::sync::Arc;

fn flatten(node: &Row) -> Vec<String> {
    let mut names = vec![node.get("Name").unwrap().wire_text().to_string()];
    for child in node.payload("Children") {
        names.extend(flatten(&child));
    }
    names
}

#[tokio::test]
async fn tree_read_stops_on_a_cycle() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    for sql in [
        "INSERT INTO categories (id, parent_category_id, name) VALUES (1, 3, 'food')",
        "INSERT INTO categories (id, parent_category_id, name) VALUES (2, 1, 'fruit')",
        "INSERT INTO categories (id, parent_category_id, name) VALUES (3, 2, 'apple')",
    ] {
        command.execute_native(&Native::new(sql), None).await.unwrap();
    }
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "categories".into();
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(ReadonlyRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = ReadonlyRepository::<Row>::new(connection, options()).unwrap();
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
        .read_single_where(&Where::new(Operator::equals("Id", 1i64)), None)
        .await
        .unwrap()
        .unwrap();
    let children = food.payload("Children");
    assert_eq!(children[0].get("Name").unwrap().wire_text(), "fruit");
    let apples = children[0].payload("Children");
    assert_eq!(apples[0].get("Name").unwrap().wire_text(), "apple");
    assert!(apples[0].payload("Children").is_empty());
}

#[tokio::test]
async fn tree_read_returns_each_row_once_under_the_roots() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    for sql in [
        "INSERT INTO categories (id, parent_category_id, name) VALUES (1, NULL, 'food')",
        "INSERT INTO categories (id, parent_category_id, name) VALUES (2, 1, 'fruit')",
        "INSERT INTO categories (id, parent_category_id, name) VALUES (3, 2, 'apple')",
    ] {
        command.execute_native(&Native::new(sql), None).await.unwrap();
    }
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "categories".into();
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(ReadonlyRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = ReadonlyRepository::<Row>::new(connection, options()).unwrap();
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
    assert_eq!(
        roots.iter().map(|row| row.get("Name").unwrap().wire_text()).collect::<Vec<_>>(),
        ["food"]
    );
    assert_eq!(flatten(&roots[0]), ["food", "fruit", "apple"]);
    let fruit = repo
        .read_single_where(&Where::new(Operator::equals("Id", 2i64)), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fruit.get("Name").unwrap().wire_text(), "fruit");
    let apples = fruit.payload("Children");
    assert_eq!(apples[0].get("Name").unwrap().wire_text(), "apple");
}

#[tokio::test]
async fn tree_read_returns_the_composite_root() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id)
            )",
            ),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO region_categories (region_id, category_id, name) VALUES (1, 10, 'food')"),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new(
                "INSERT INTO region_categories (region_id, category_id, parent_region_id, parent_category_id, name) VALUES (1, 11, 1, 10, 'fruit')",
            ),
            None,
        )
        .await
        .unwrap();
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = ReadonlyRepositoryMappingOptions::default();
        options.entity = "region_categories".into();
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(ReadonlyRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = ReadonlyRepository::<Row>::new(connection, options()).unwrap();
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
    assert_eq!(
        roots.iter().map(|row| row.get("Name").unwrap().wire_text()).collect::<Vec<_>>(),
        ["food"]
    );
    let children = roots[0].payload("Children");
    assert_eq!(children[0].get("Name").unwrap().wire_text(), "fruit");
}

#[tokio::test]
async fn tree_create_then_read_matches() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "categories".into();
        options.ids = vec!["Id".into()];
        options.create_properties = Some(vec!["Name".into(), "ParentCategoryId".into()]);
        options.read_properties = vec!["Id".into(), "Name".into(), "ParentCategoryId".into()];
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(CRUDRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = CRUDRepository::<Row>::new(connection, options())
        .unwrap()
        .with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(
            None,
            None,
            Some(Arc::new(TreeCreateEnrichmentStrategy::new())),
            false,
        )));
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::clone(&via),
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let apple: Row = [("Name".into(), datasources::Value::from("apple"))].into_iter().collect();
    let fruit: Row = [
        ("Name".into(), datasources::Value::from("fruit")),
        ("Children".into(), encode_rows(&[apple])),
    ]
    .into_iter()
    .collect();
    repo.create(
        [
            ("Name".into(), datasources::Value::from("food")),
            ("Children".into(), encode_rows(&[fruit])),
        ]
        .into_iter()
        .collect(),
    )
    .await
    .unwrap();
    let food = repo
        .read_single_where(&Where::new(Operator::equals("Name", "food")), None)
        .await
        .unwrap()
        .unwrap();
    let children = food.payload("Children");
    assert_eq!(children[0].get("Name").unwrap().wire_text(), "fruit");
    assert_eq!(
        children[0].payload("Children")[0].get("Name").unwrap().wire_text(),
        "apple"
    );
}

#[tokio::test]
async fn tree_create_throws_on_a_cycle() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "categories".into();
        options.ids = vec!["Id".into()];
        options.create_properties = Some(vec!["Name".into(), "ParentCategoryId".into()]);
        options.read_properties = vec!["Id".into(), "Name".into(), "ParentCategoryId".into()];
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(CRUDRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = CRUDRepository::<Row>::new(connection, options())
        .unwrap()
        .with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(
            None,
            None,
            Some(Arc::new(TreeCreateEnrichmentStrategy::new())),
            false,
        )));
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentCategoryId",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::clone(&via),
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let food: Row = [("Name".into(), datasources::Value::from("food"))].into_iter().collect();
    let apple: Row = [
        ("Name".into(), datasources::Value::from("apple")),
        ("Children".into(), encode_rows(&[food.clone()])),
    ]
    .into_iter()
    .collect();
    let fruit: Row = [
        ("Name".into(), datasources::Value::from("fruit")),
        ("Children".into(), encode_rows(&[apple])),
    ]
    .into_iter()
    .collect();
    let error = repo
        .create(
            [
                ("Name".into(), datasources::Value::from("food")),
                ("Children".into(), encode_rows(&[fruit])),
            ]
            .into_iter()
            .collect(),
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("food"), "{error}");
}

#[tokio::test]
async fn tree_create_stamps_composite_parents() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id),
                FOREIGN KEY (parent_region_id, parent_category_id) REFERENCES region_categories (region_id, category_id)
            )",
            ),
            None,
        )
        .await
        .unwrap();
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let options = || {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "region_categories".into();
        options.ids = vec!["RegionId".into(), "CategoryId".into()];
        options.create_properties = Some(vec![
            "RegionId".into(),
            "CategoryId".into(),
            "ParentRegionId".into(),
            "ParentCategoryId".into(),
            "Name".into(),
        ]);
        options.read_properties = vec![
            "RegionId".into(),
            "CategoryId".into(),
            "ParentRegionId".into(),
            "ParentCategoryId".into(),
            "Name".into(),
        ];
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let via = Arc::new(CRUDRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, options()).unwrap());
    let mut repo = CRUDRepository::<Row>::new(connection, options())
        .unwrap()
        .with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(
            None,
            None,
            Some(Arc::new(TreeCreateEnrichmentStrategy::new())),
            false,
        )));
    repo.add_enriched(
        Enrichment::new(
            "Children",
            "RegionId",
            "ParentRegionId",
            None,
            EnrichmentPropertyType::List,
        )
        .with_keys(&[("RegionId", "ParentRegionId"), ("CategoryId", "ParentCategoryId")]),
        Arc::clone(&via),
        Some(Arc::new(TreeReadonlyEnrichmentStrategy)),
    );
    let coast: Row = [
        ("RegionId".into(), datasources::Value::from(1i64)),
        ("CategoryId".into(), datasources::Value::from(2i64)),
        ("Name".into(), datasources::Value::from("coast")),
    ]
    .into_iter()
    .collect();
    repo.create(
        [
            ("RegionId".into(), datasources::Value::from(1i64)),
            ("CategoryId".into(), datasources::Value::from(1i64)),
            ("Name".into(), datasources::Value::from("west")),
            ("Children".into(), encode_rows(&[coast])),
        ]
        .into_iter()
        .collect(),
    )
    .await
    .unwrap();
    let west = repo
        .read_single_where(&Where::new(Operator::equals("Name", "west")), None)
        .await
        .unwrap()
        .unwrap();
    let child = &west.payload("Children")[0];
    assert_eq!(child.get("Name").unwrap().wire_text(), "coast");
    assert_eq!(child.get("ParentRegionId"), Some(&datasources::Value::from(1i64)));
    assert_eq!(child.get("ParentCategoryId"), Some(&datasources::Value::from(1i64)));
}

#[tokio::test]
async fn graph_create_writes_a_loop_then_read() {
    let connection = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    connection.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&connection));
    command
        .execute_native(
            &Native::new("CREATE TABLE nodes (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT NOT NULL)"),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new(
                "CREATE TABLE edges (id INTEGER PRIMARY KEY AUTOINCREMENT, from_node_id INTEGER NOT NULL, to_node_id INTEGER NOT NULL)",
            ),
            None,
        )
        .await
        .unwrap();
    let mappings = Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let node_options = || {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "nodes".into();
        options.ids = vec!["Id".into()];
        options.create_properties = Some(vec!["Value".into()]);
        options.read_properties = vec!["Id".into(), "Value".into()];
        options.property_mappings = Some(Arc::clone(&mappings) as _);
        options
    };
    let mut edge_options = CRUDRepositoryOptions::default();
    edge_options.entity = "edges".into();
    edge_options.ids = vec!["Id".into()];
    edge_options.create_properties = Some(vec!["FromNodeId".into(), "ToNodeId".into()]);
    edge_options.read_properties = vec!["Id".into(), "FromNodeId".into(), "ToNodeId".into()];
    edge_options.property_mappings = Some(Arc::clone(&mappings) as _);
    let via = Arc::new(CRUDRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, node_options()).unwrap());
    let link: Arc<dyn ICRUDRepository> = Arc::new(
        CRUDRepository::<Row>::new(Arc::clone(&connection) as Arc<dyn Connection>, edge_options).unwrap(),
    );
    let mut repo = CRUDRepository::<Row>::new(connection, node_options())
        .unwrap()
        .with_write(Arc::new(DefaultWriteEnrichmentStrategy::new(
            None,
            None,
            Some(Arc::new(GraphCreateEnrichmentStrategy::new(Arc::clone(&link)))),
            false,
        )));
    repo.add_enriched(
        Enrichment::new("Edges", "Id", "Id", None, EnrichmentPropertyType::List)
            .linked(Association::new("FromNodeId", "ToNodeId")),
        Arc::clone(&via),
        Some(Arc::new(GraphReadonlyEnrichmentStrategy::new(Arc::clone(&link)))),
    );
    let mut a: Row = [("Value".into(), datasources::Value::from("a"))].into_iter().collect();
    let b: Row = [
        ("Value".into(), datasources::Value::from("b")),
        ("Edges".into(), encode_rows(&[a.clone()])),
    ]
    .into_iter()
    .collect();
    a.insert("Edges".into(), encode_rows(&[b]));
    repo.create(a).await.unwrap();
    let loaded_a = repo
        .read_single_where(&Where::new(Operator::equals("Value", "a")), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded_a.payload("Edges")[0].get("Value").unwrap().wire_text(), "b");
    let loaded_b = repo
        .read_single_where(&Where::new(Operator::equals("Value", "b")), None)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(loaded_b.payload("Edges")[0].get("Value").unwrap().wire_text(), "a");
}
