use futures_util::TryStreamExt;
use datasources::*;
use datasources_memory::MemoryDriver;
use std::sync::Arc;

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

fn types_repo(connection: Arc<dyn Connection>) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "UserType".into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(columns()));
    CRUDRepository::new(connection, options).unwrap()
}

fn parents_repo(connection: Arc<dyn Connection>) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(Arc::new(columns()));
    CRUDRepository::new(connection, options).unwrap()
}

#[tokio::test]
async fn read_with_where_clause_and_custom_flat_to_nested_object_mapping_options() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let types = Arc::new(types_repo(connection.clone()));
    let mut created_type = record(&[("Name", Value::from("person"))]);
    let created_type = types.create(&mut created_type).await.unwrap();
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
    let mut parent = record(&[("Type", created_type.get("Id").unwrap().clone())]);
    parents.create(&mut parent).await.unwrap();
    let enriched = parents.read_single(None, None, None).await.unwrap();

    let command = Arc::new(MockCommand::new());
    command.reader_rows.lock().unwrap().extend([
        record(&[
            ("Id", Value::Int(1)),
            ("TypeName", Value::from("person")),
            ("ChildId", Value::Int(10)),
            ("Label", Value::from("a")),
        ]),
        record(&[
            ("Id", Value::Int(1)),
            ("TypeName", Value::from("person")),
            ("ChildId", Value::Int(11)),
            ("Label", Value::from("b")),
        ]),
    ]);
    let mut join_repo = parents_repo(MockConnection::with_command_provider(
        command,
        "memory",
    ));
    join_repo.add_enriched(
        Enrichment::new(
            "TypeName",
            "Type",
            "Id",
            Some("Name".into()),
            EnrichmentPropertyType::Object,
        ),
        as_readonly(types),
        None,
    );
    join_repo.add_enriched(
        Enrichment::new(
            "Children",
            "Id",
            "ParentId",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::new(CRUDRepository::new(
            Arc::new(StubConnection::new("memory")),
            CRUDRepositoryOptions::default(),
        ).unwrap()),
        Some(Arc::new(NestedToObjectEnrichmentStrategy::new(TypeMap::new(
            "Id",
            vec![
                ("Id".into(), TypeMapField::column("Id")),
                ("TypeName".into(), TypeMapField::column("TypeName")),
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
        )))),
    );
    let where_clause = Where::new(Operator::equals("Parent.Id", Value::Int(1)));
    let join = SelectStatement::new(SelectStatementOptions {
        joins: &[Join::left("Child", "Parent.Id", "Child.ParentId")],
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new(
            "Parent",
            &[
                "Parent.Id AS Id",
                "Parent.TypeName AS TypeName",
                "Child.Id AS ChildId",
                "Child.Label AS Label",
            ],
        )
    });
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity_mapper = Some(Arc::new(join_entity()));
    let items = join_repo
        .read(Some(&join), None, Some(&options))
        .try_collect::<Vec<_>>().await
        .unwrap();
    assert_eq!(
        enriched.as_ref().unwrap().get("TypeName"),
        Some(&Value::from("person"))
    );
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("Id"), Some(&Value::Int(1)));
    assert_eq!(items[0].get("TypeName"), Some(&Value::from("person")));
    let children = items[0].get("Children").unwrap().wire_text();
    assert!(children.contains("Label=a"));
    assert!(children.contains("Label=b"));
}
