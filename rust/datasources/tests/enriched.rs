use datasources::*;
use datasources_memory::MemoryProvider;
use std::sync::Arc;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

async fn setup(rows: &mut [Row]) -> Arc<dyn Connection> {
    let connection = MemoryProvider.connection(None);
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

fn crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    CRUDRepository::new(connection, options).unwrap()
}

fn readonly(connection: Arc<dyn Connection>, entity: &str) -> ReadonlyRepository {
    let mut options = ReadonlyRepositoryOptions::default();
    options.entity = entity.into();
    ReadonlyRepository::new(connection, options).unwrap()
}

fn num(row: &Row, key: &str) -> i64 {
    match row.get(key) {
        Some(Value::Int(id)) => *id,
        _ => panic!("{key}"),
    }
}

async fn drop_row(connection: &Arc<dyn Connection>, row: &Row) {
    connection
        .create_command()
        .execute("delete", Some(&[Parameter::new("id", row.get("id").unwrap().clone())]))
        .await
        .unwrap();
}

fn open_contacts(
    mut contacts: CRUDRepository,
    types: Arc<dyn IReadonlyRepository>,
    sources: Arc<dyn IReadonlyRepository>,
    address_types: Arc<dyn IReadonlyRepository>,
    phone_types: Arc<dyn IReadonlyRepository>,
    mut addresses: CRUDRepository,
    mut phones: CRUDRepository,
) -> CRUDRepository {
    addresses.add_enriched(
        Enrichment::new("address_type_name", "address_type_id", "id", Some("name".into()), false),
        address_types,
        None,
    );
    phones.add_enriched(
        Enrichment::new("phone_type_name", "phone_type_id", "id", Some("name".into()), false),
        phone_types,
        None,
    );
    contacts
        .add_enriched(
            Enrichment::new("contact_type_name", "contact_type_id", "id", Some("name".into()), false),
            types,
            None,
        )
        .add_enriched(
            Enrichment::new("contact_source_name", "contact_source_id", "id", Some("name".into()), false),
            sources,
            None,
        )
        .add_enriched(
            Enrichment::new("addresses", "id", "contact_id", None, true),
            Arc::new(addresses),
            None,
        )
        .add_enriched(
            Enrichment::new("phones", "id", "contact_id", None, true),
            Arc::new(phones),
            None,
        );
    contacts
}

#[tokio::test]
async fn enriched_fills_lookup_select_and_leaves_missing_default() {
    let mut seeds = [
        record(&[("name", Value::from("person"))]),
        record(&[("name", Value::from("gone"))]),
    ];
    let types_connection = setup(&mut seeds).await;
    let person = seeds[0].clone();
    let gone = seeds[1].clone();
    drop_row(&types_connection, &gone).await;
    let mut parents = [
        record(&[("type_id", person.get("id").unwrap().clone())]),
        record(&[("type_id", gone.get("id").unwrap().clone())]),
    ];
    let connection = setup(&mut parents).await;
    let mut parents_repo = readonly(connection, "Parent");
    let types_repo = Arc::new(readonly(types_connection, "TypeRow"));
    parents_repo.add_enriched(
        Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
        types_repo,
        None,
    );
    let items = parents_repo.read(None, None).await.unwrap();
    assert_eq!(items[0]["type_name"], Value::from("person"));
    assert!(!items[1].contains_key("type_name") || items[1]["type_name"] == Value::from(""));
}

#[tokio::test]
async fn deep_create_writes_children_stamps_fk() {
    let connection = setup(&mut []).await;
    let mut parents = crud(connection.clone(), "Parent");
    let types = Arc::new(crud(connection.clone(), "TypeRow"));
    let children = Arc::new(crud(connection, "Child"));
    types
        .create(&mut record(&[("name", Value::from("person"))]))
        .await
        .unwrap();
    let kind = types.read(None, None).await.unwrap();
    parents
        .add_enriched(
            Enrichment::new("type_name", "type_id", "id", Some("name".into()), false),
            types.clone(),
            None,
        )
        .add_enriched(
            Enrichment::new("children", "id", "parent_id", None, true),
            children.clone(),
            None,
        );
    parents
        .create(&mut record(&[
            ("type_id", kind[0].get("id").unwrap().clone()),
            ("label", Value::from("p")),
            (
                "children",
                encode_rows(&[
                    record(&[("label", Value::from("a"))]),
                    record(&[("label", Value::from("b"))]),
                ]),
            ),
        ]))
        .await
        .unwrap();
    let stored_parents = parents.read(None, None).await.unwrap();
    let stored = children.read(None, None).await.unwrap();
    assert!(num(&stored_parents[0], "id") > 0);
    assert_eq!(stored.len(), 2);
    assert_eq!(stored[0]["parent_id"], stored_parents[0]["id"]);
    assert_eq!(stored[1]["parent_id"], stored_parents[0]["id"]);
    assert_eq!(types.read(None, None).await.unwrap()[0]["name"], Value::from("person"));
}

#[tokio::test]
async fn contacts_service_fills_names_and_child_lists() {
    let mut person = [record(&[("name", Value::from("person"))])];
    let types_connection = setup(&mut person).await;
    let mut source = [record(&[("name", Value::from("import"))])];
    let sources_connection = setup(&mut source).await;
    let mut home = [record(&[("name", Value::from("Home"))])];
    let address_types_connection = setup(&mut home).await;
    let mut work = [record(&[("name", Value::from("Work"))])];
    let phone_types_connection = setup(&mut work).await;
    let connection = setup(&mut []).await;
    let contacts = crud(connection.clone(), "Contact");
    let addresses = crud(connection.clone(), "Address");
    let phones = crud(connection, "Phone");
    contacts
        .create(&mut record(&[
            ("contact_type_id", person[0].get("id").unwrap().clone()),
            ("contact_source_id", source[0].get("id").unwrap().clone()),
            ("first_name", Value::from("Other")),
            ("label", Value::from("other")),
        ]))
        .await
        .unwrap();
    let other = contacts.read(None, None).await.unwrap().into_iter().next().unwrap();
    assert!(num(&other, "id") > 0);
    addresses
        .create(&mut record(&[
            ("contact_id", other.get("id").unwrap().clone()),
            ("line1", Value::from("other")),
            ("city", Value::from("x")),
            ("address_type_id", home[0].get("id").unwrap().clone()),
            ("label", Value::from("stray")),
        ]))
        .await
        .unwrap();
    let service = open_contacts(
        contacts,
        Arc::new(readonly(types_connection, "ContactType")),
        Arc::new(readonly(sources_connection, "ContactSource")),
        Arc::new(readonly(address_types_connection, "AddressType")),
        Arc::new(readonly(phone_types_connection, "PhoneType")),
        addresses,
        phones,
    );
    service
        .create(&mut record(&[
            ("contact_type_id", person[0].get("id").unwrap().clone()),
            ("contact_source_id", source[0].get("id").unwrap().clone()),
            ("first_name", Value::from("Ada")),
            ("label", Value::from("ada")),
            (
                "addresses",
                encode_rows(&[record(&[
                    ("line1", Value::from("1 Main")),
                    ("city", Value::from("Paris")),
                    ("address_type_id", home[0].get("id").unwrap().clone()),
                ])]),
            ),
            (
                "phones",
                encode_rows(&[record(&[
                    ("number", Value::from("555")),
                    ("phone_type_id", work[0].get("id").unwrap().clone()),
                ])]),
            ),
        ]))
        .await
        .unwrap();
    let contact = service
        .read(None, None)
        .await
        .unwrap()
        .into_iter()
        .find(|item| item.get("first_name") == Some(&Value::from("Ada")))
        .unwrap();
    assert!(num(&contact, "id") > 0);
    assert_ne!(contact["id"], other["id"]);
    assert_eq!(contact["contact_type_name"], Value::from("person"));
    assert_eq!(contact["contact_source_name"], Value::from("import"));
    assert!(contact["addresses"].wire_text().contains("line1=1 Main"));
    assert!(contact["addresses"].wire_text().contains("address_type_name=Home"));
    assert!(!contact["addresses"].wire_text().contains("line1=other"));
    assert!(contact["phones"].wire_text().contains("number=555"));
    assert!(contact["phones"].wire_text().contains("phone_type_name=Work"));
}

#[tokio::test]
async fn deep_update_replace_syncs_children() {
    let connection = setup(&mut []).await;
    let mut parents = crud(connection.clone(), "Parent");
    let children = Arc::new(crud(connection, "Child"));
    parents
        .create(&mut record(&[("label", Value::from("p"))]))
        .await
        .unwrap();
    let parent = &parents.read(None, None).await.unwrap()[0].clone();
    let parent_id = parent.get("id").unwrap().clone();
    children
        .create(&mut record(&[("parent_id", parent_id.clone()), ("label", Value::from("a"))]))
        .await
        .unwrap();
    children
        .create(&mut record(&[("parent_id", parent_id.clone()), ("label", Value::from("b"))]))
        .await
        .unwrap();
    let kept = children
        .read(None, None)
        .await
        .unwrap()
        .into_iter()
        .find(|row| row.get("label") == Some(&Value::from("a")))
        .unwrap();
    parents.add_enriched(
        Enrichment::new("children", "id", "parent_id", None, true),
        children.clone(),
        None,
    );
    parents
        .update(
            &record(&[
                ("id", parent_id.clone()),
                ("label", Value::from("p")),
                (
                    "children",
                    encode_rows(&[
                        record(&[("id", kept.get("id").unwrap().clone()), ("label", Value::from("aa"))]),
                        record(&[("label", Value::from("c"))]),
                    ]),
                ),
            ]),
            None,
        )
        .await
        .unwrap();
    let stored = children.read(None, None).await.unwrap();
    assert_eq!(stored.len(), 2);
    assert_eq!(
        stored.iter().find(|row| row.get("id") == kept.get("id")).unwrap()["label"],
        Value::from("aa")
    );
    assert_eq!(
        stored.iter().find(|row| row.get("id") != kept.get("id")).unwrap()["label"],
        Value::from("c")
    );
}

#[tokio::test]
async fn deep_delete_cascades_children_then_parent() {
    let connection = setup(&mut []).await;
    let parents = crud(connection.clone(), "Parent");
    let mut children_repo = crud(connection.clone(), "Child");
    let notes = Arc::new(crud(connection, "Note"));
    parents
        .create(&mut record(&[("label", Value::from("p"))]))
        .await
        .unwrap();
    let parent_id = parents.read(None, None).await.unwrap()[0].get("id").unwrap().clone();
    children_repo
        .create(&mut record(&[("parent_id", parent_id.clone()), ("label", Value::from("a"))]))
        .await
        .unwrap();
    let child_id = children_repo.read(None, None).await.unwrap()[0].get("id").unwrap().clone();
    notes
        .create(&mut record(&[("child_id", child_id), ("text", Value::from("n"))]))
        .await
        .unwrap();
    notes
        .create(&mut record(&[("text", Value::from("other")), ("label", Value::from("keep"))]))
        .await
        .unwrap();
    children_repo.add_enriched(Enrichment::new("notes", "id", "child_id", None, true), notes.clone(), None);
    let children = Arc::new(children_repo);
    let mut parents = parents;
    parents.add_enriched(Enrichment::new("children", "id", "parent_id", None, true), children.clone(), None);
    parents
        .delete(&record(&[("id", parent_id)]), None)
        .await
        .unwrap();
    assert!(parents.read(None, None).await.unwrap().is_empty());
    assert!(children.read(None, None).await.unwrap().is_empty());
    assert_eq!(notes.read(None, None).await.unwrap()[0]["text"], Value::from("other"));
}

#[tokio::test]
async fn deep_many_readonly_via_throws() {
    let connection = setup(&mut []).await;
    let mut parents = crud(connection.clone(), "Parent");
    parents.add_enriched(
        Enrichment::new("children", "id", "parent_id", None, true),
        Arc::new(readonly(connection, "Child")),
        None,
    );
    let error = parents
        .create(&mut record(&[(
            "children",
            encode_rows(&[record(&[("label", Value::from("a"))])]),
        )]))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("children"));
}

#[tokio::test]
async fn bulk_create_writes_parents_and_children() {
    let connection = setup(&mut []).await;
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    let mut parents = BulkCRUDRepository::new(connection.clone(), options).unwrap();
    let mut child_options = CRUDRepositoryOptions::default();
    child_options.entity = "Child".into();
    let children = Arc::new(BulkCRUDRepository::new(connection, child_options).unwrap());
    parents.add_enriched(Enrichment::new("children", "id", "parent_id", None, true), children.clone(), None);
    let response = parents
        .create_bulk(&mut [
            record(&[(
                "children",
                encode_rows(&[record(&[("label", Value::from("a"))])]),
            )]),
            record(&[(
                "children",
                encode_rows(&[
                    record(&[("label", Value::from("b"))]),
                    record(&[("label", Value::from("c"))]),
                ]),
            )]),
        ])
        .await
        .unwrap();
    assert_eq!(parents.read(None, None).await.unwrap().len(), 2);
    assert_eq!(children.read(None, None).await.unwrap().len(), 3);
    assert_eq!(response.rows_affected, 2);
}

