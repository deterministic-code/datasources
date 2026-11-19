use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use datasources::{
    Casing, ChangeCasingMapper, Command, Connection, Entity, Enrichment, EnrichmentPropertyType, Native,
    Operator, ReadonlyRepository, ReadonlyRepositoryMappingOptions, Row, TreeReadonlyEnrichmentStrategy,
    Where,
};
use std::sync::Arc;

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
