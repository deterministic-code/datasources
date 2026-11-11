use datasources::*;
use datasources_memory::MemoryDriver;
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
#[allow(non_snake_case)]
struct Parent {
    Id: i64,
    TypeName: String,
    Settings: HashMap<String, String>,
    Children: Vec<Child>,
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
#[allow(non_snake_case)]
struct Child {
    Id: i64,
    Label: String,
}

impl SchemaObject for Parent {}
impl SchemaObject for Child {}

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

fn columns() -> TransformMapper<String, String> {
    TransformMapper::new(
        |name: String| {
            let references = DictionaryMapper::new([("Type".to_string(), "type_id".to_string())]);
            let snake = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
            let mapped = references.map(name.clone());
            if mapped != name {
                mapped
            } else {
                snake.map(name)
            }
        },
        |name: String| {
            let references = DictionaryMapper::new([("Type".to_string(), "type_id".to_string())]);
            let snake = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
            let from = references.reverse_map(name.clone());
            if from != name {
                from
            } else {
                snake.reverse_map(name)
            }
        },
    )
}

fn join_entity() -> DictionaryMapper<String> {
    DictionaryMapper::new([
        ("Parent".to_string(), "parent".to_string()),
        ("Child".to_string(), "child".to_string()),
    ])
}

fn parent_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(columns()));
    options
}

fn types_repo(connection: Arc<dyn Connection>) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "UserType".into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(columns()));
    CRUDRepository::<Row>::new(connection, options).unwrap()
}

fn parents_repo(connection: Arc<dyn Connection>) -> CRUDRepository {
    CRUDRepository::<Row>::new(connection, parent_options()).unwrap()
}

fn join_parents_repo(connection: Arc<dyn Connection>) -> CRUDRepository<Parent> {
    CRUDRepository::<Parent>::new(connection, parent_options()).unwrap()
}

#[tokio::test]
async fn read_with_where_clause_and_custom_flat_to_nested_object_mapping_options() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let types = Arc::new(types_repo(connection.clone()));
    let created_type = types
        .create(record(&[("Name", Value::from("person"))]))
        .await
        .unwrap();
    let mut parents = parents_repo(connection);
    parents.add_enriched(
        Enrichment::new(
            "TypeName",
            "Type",
            "Id",
            Some("Name".into()),
            EnrichmentPropertyType::Object,
        ),
        as_readonly(types.clone()),
        None,
    );
    let parent = record(&[("Type", created_type.get("Id").unwrap().clone())]);
    parents.create(parent).await.unwrap();
    let enriched = parents.read_single(None, None).await.unwrap();

    let command = Arc::new(MockCommand::new());
    command.reader_rows.lock().unwrap().extend([
        record(&[
            ("Id", Value::Int(1)),
            ("TypeName", Value::from("person")),
            ("ChildId", Value::Int(10)),
            ("Label", Value::from("a")),
            ("SettingName", Value::from("lives")),
            ("SettingValue", Value::from("3")),
        ]),
        record(&[
            ("Id", Value::Int(1)),
            ("TypeName", Value::from("person")),
            ("ChildId", Value::Int(11)),
            ("Label", Value::from("b")),
            ("SettingName", Value::from("level")),
            ("SettingValue", Value::from("2")),
        ]),
    ]);
    let join_repo = join_parents_repo(MockConnection::with_command_provider(command, "memory"));
    let where_clause = Where::new(Operator::equals("Parent.Id", Value::Int(1)));
    let join = SelectStatement::new(SelectStatementOptions {
        joins: &[Join::left("Child", "Parent.Id", "Child.ParentId")],
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new(
            "Parent",
            &["Parent.Id", "Parent.TypeName", "Child.Id", "Child.Label"],
        )
    });
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity_mapper = Some(Arc::new(join_entity()));
    options.enrichment = Some(Arc::new(NestedToObjectEnrichmentStrategy::new(
        TypeMap::new(
            "Id",
            vec![
                ("Id".into(), TypeMapField::column("Id")),
                ("TypeName".into(), TypeMapField::column("TypeName")),
                (
                    "Settings".into(),
                    TypeMapField::dictionary("SettingName", "SettingValue"),
                ),
                (
                    "Children".into(),
                    TypeMapField::Group(TypeMapGroup::new(
                        "ChildId",
                        vec![
                            ("Id".into(), TypeMapField::column("ChildId")),
                            ("Label".into(), TypeMapField::column("Label")),
                        ],
                    )),
                ),
            ],
        ),
    )));
    let items = join_repo
        .read(Some(&join), Some(&options))
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        enriched.as_ref().unwrap().get("TypeName"),
        Some(&Value::from("person"))
    );
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].Id, 1);
    assert_eq!(items[0].TypeName, "person");
    assert_eq!(
        items[0]
            .Children
            .iter()
            .map(|child| child.Label.as_str())
            .collect::<Vec<_>>(),
        ["a", "b"]
    );
    assert_eq!(items[0].Settings.get("lives"), Some(&"3".to_string()));
    assert_eq!(items[0].Settings.get("level"), Some(&"2".to_string()));
}
