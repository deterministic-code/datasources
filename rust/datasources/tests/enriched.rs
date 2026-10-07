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

fn mapped_crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.property_mappings = Some(std::sync::Arc::new(columns()));
    CRUDRepository::new(connection, options).unwrap()
}

fn create_lookup_repo(connection: Arc<dyn Connection>, entity: &str) -> ReadonlyRepository {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = entity.into();
    options.entity_mapper = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.property_mappings = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    ReadonlyRepository::new(connection, options).unwrap()
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

fn mapped(connection: Arc<dyn Connection>, entity: &str) -> ReadonlyRepository {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = entity.into();
    options.property_mappings = Some(std::sync::Arc::new(columns()));
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
        .execute(
            "delete",
            Some(&[Parameter::new("id", row.get("id").unwrap().clone())]),
        )
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
        Enrichment::new("AddressType", "AddressType", "Id", None, datasources::EnrichmentPropertyType::Object),
        address_types,
        None,
    );
    phones.add_enriched(
        Enrichment::new("PhoneType", "PhoneType", "Id", None, datasources::EnrichmentPropertyType::Object),
        phone_types,
        None,
    );
    contacts
        .add_enriched(
            Enrichment::new("ContactType", "ContactType", "Id", None, datasources::EnrichmentPropertyType::Object),
            types,
            None,
        )
        .add_enriched(
            Enrichment::new("ContactSource", "ContactSource", "Id", None, datasources::EnrichmentPropertyType::Object),
            sources,
            None,
        )
        .add_enriched(
            Enrichment::new("Addresses", "Id", "Contact", None, datasources::EnrichmentPropertyType::List),
            Arc::new(addresses),
            None,
        )
        .add_enriched(
            Enrichment::new("Phones", "Id", "Contact", None, datasources::EnrichmentPropertyType::List),
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
    let mut parents_repo = mapped(connection, "Parent");
    let types_repo = Arc::new(mapped(types_connection, "UserType"));
    parents_repo.add_enriched(
        Enrichment::new("TypeName", "Type", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
        types_repo,
        None,
    );
    let items = parents_repo.read(None, None, None).await.unwrap();
    assert_eq!(items[0]["TypeName"], Value::from("person"));
    assert!(!items[1].contains_key("TypeName") || items[1]["TypeName"] == Value::from(""));
}

#[tokio::test]
async fn deep_create_writes_children_stamps_fk() {
    let connection = setup(&mut []).await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let types = Arc::new(mapped_crud(connection.clone(), "UserType"));
    let children = Arc::new(mapped_crud(connection, "Child"));
    let kind = types
        .create(&mut record(&[("Name", Value::from("person"))]))
        .await
        .unwrap();
    assert!(num(&kind, "Id") > 0);
    parents
        .add_enriched(
            Enrichment::new("TypeName", "Type", "Id", Some("Name".into()), datasources::EnrichmentPropertyType::Object),
            types.clone(),
            None,
        )
        .add_enriched(
            Enrichment::new("Children", "Id", "ParentId", None, datasources::EnrichmentPropertyType::List),
            children.clone(),
            None,
        );
    let created = parents
        .create(&mut record(&[
            ("Type", kind.get("Id").unwrap().clone()),
            ("Label", Value::from("p")),
            (
                "Children",
                encode_rows(&[
                    record(&[("Label", Value::from("a"))]),
                    record(&[("Label", Value::from("b"))]),
                ]),
            ),
        ]))
        .await
        .unwrap();
    assert!(num(&created, "Id") > 0);
    let stored_parents = parents.read(None, None, None).await.unwrap();
    let stored = children.read(None, None, None).await.unwrap();
    assert_eq!(stored_parents[0]["Id"], created["Id"]);
    assert_eq!(stored.len(), 2);
    assert_eq!(stored[0]["ParentId"], stored_parents[0]["Id"]);
    assert_eq!(stored[1]["ParentId"], stored_parents[0]["Id"]);
    assert_eq!(
        types.read(None, None, None).await.unwrap()[0]["Name"],
        Value::from("person")
    );
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
    let contacts = domain_crud(connection.clone(), "Contact");
    let addresses = domain_crud(connection.clone(), "Address");
    let phones = domain_crud(connection, "Phone");
    let other = contacts
        .create(&mut record(&[
            ("ContactType", person[0].get("id").unwrap().clone()),
            ("ContactSource", source[0].get("id").unwrap().clone()),
            ("FirstName", Value::from("Other")),
        ]))
        .await
        .unwrap();
    assert!(num(&other, "Id") > 0);
    let stray = addresses
        .create(&mut record(&[
            ("Contact", other.get("Id").unwrap().clone()),
            ("Line1", Value::from("other")),
            ("City", Value::from("x")),
            ("AddressType", home[0].get("id").unwrap().clone()),
        ]))
        .await
        .unwrap();
    assert!(num(&stray, "Id") > 0);
    let service = open_contacts(
        contacts,
        Arc::new(create_lookup_repo(types_connection, "ContactType")),
        Arc::new(create_lookup_repo(sources_connection, "ContactSource")),
        Arc::new(create_lookup_repo(address_types_connection, "AddressType")),
        Arc::new(create_lookup_repo(phone_types_connection, "PhoneType")),
        addresses,
        phones,
    );
    let created = service
        .create(&mut record(&[
            ("ContactType", person[0].get("id").unwrap().clone()),
            ("ContactSource", source[0].get("id").unwrap().clone()),
            ("FirstName", Value::from("Ada")),
            (
                "Addresses",
                encode_rows(&[record(&[
                    ("Line1", Value::from("1 Main")),
                    ("City", Value::from("Paris")),
                    ("AddressType", home[0].get("id").unwrap().clone()),
                ])]),
            ),
            (
                "Phones",
                encode_rows(&[record(&[
                    ("Number", Value::from("555")),
                    ("PhoneType", work[0].get("id").unwrap().clone()),
                ])]),
            ),
        ]))
        .await
        .unwrap();
    assert!(num(&created, "Id") > 0);
    let contact = service
        .read(None, None, None)
        .await
        .unwrap()
        .into_iter()
        .find(|item| item.get("FirstName") == Some(&Value::from("Ada")))
        .unwrap();
    assert_eq!(contact["Id"], created["Id"]);
    assert_ne!(contact["Id"], other["Id"]);
    assert!(contact["ContactType"].wire_text().contains("Name=person")
        || contact["ContactType"].wire_text().contains("name=person"));
    assert!(contact["ContactSource"].wire_text().contains("Name=import")
        || contact["ContactSource"].wire_text().contains("name=import"));
    let addresses = contact["Addresses"].wire_text();
    assert!(addresses.contains("Line1=1 Main"), "{addresses}");
    assert!(
        addresses.contains("Name=Home")
            || addresses.contains("name=Home")
            || addresses.contains("Name%3DHome"),
        "{addresses}"
    );
    assert!(!contact["Addresses"].wire_text().contains("Line1=other"));
    let phones = contact["Phones"].wire_text();
    assert!(phones.contains("Number=555"), "{phones}");
    assert!(
        phones.contains("Name=Work") || phones.contains("name=Work") || phones.contains("Name%3DWork"),
        "{phones}"
    );
}

fn domain_crud(connection: Arc<dyn Connection>, entity: &str) -> CRUDRepository {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = entity.into();
    options.ids = vec!["Id".into()];
    options.entity_mapper = Some(std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake)));
    options.property_mappings = Some(std::sync::Arc::new(contact_columns()));
    options.object_converter = Some(Box::new(
        MetaDataRowTypeConverter::new(Vec::new())
            .with_references(contact_references(), Box::new(contact_columns())),
    ));
    CRUDRepository::new(connection, options).unwrap()
}

fn contact_references() -> Vec<String> {
    ["Contact", "AddressType", "ContactType", "ContactSource", "PhoneType"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

fn contact_columns() -> TransformMapper<String, String> {
    let references = std::sync::Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contact_id".to_string()),
        ("AddressType".to_string(), "address_type_id".to_string()),
        ("ContactType".to_string(), "contact_type_id".to_string()),
        ("ContactSource".to_string(), "contact_source_id".to_string()),
        ("PhoneType".to_string(), "phone_type_id".to_string()),
        ("Line1".to_string(), "line1".to_string()),
        ("Line2".to_string(), "line2".to_string()),
    ]));
    let snake = std::sync::Arc::new(ChangeCasingMapper::new(Casing::Pascal, Casing::Snake));
    let forward_references = references.clone();
    let forward_snake = snake.clone();
    let reverse_references = references.clone();
    let reverse_snake = snake;
    TransformMapper::new(
        move |name: String| {
            let mapped = Mapper::map(forward_references.as_ref(), name.clone());
            if mapped != name {
                mapped
            } else {
                Mapper::map(forward_snake.as_ref(), name)
            }
        },
        move |name: String| {
            let from = Mapper::reverse_map(reverse_references.as_ref(), name.clone());
            if from != name {
                from
            } else {
                Mapper::reverse_map(reverse_snake.as_ref(), name)
            }
        },
    )
}

#[tokio::test]
async fn deep_update_replace_syncs_children() {
    let connection = setup(&mut []).await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    let children = Arc::new(mapped_crud(connection, "Child"));
    let parent = parents
        .create(&mut record(&[("Label", Value::from("p"))]))
        .await
        .unwrap();
    assert!(num(&parent, "Id") > 0);
    let parent_id = parent.get("Id").unwrap().clone();
    let kept = children
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("Label", Value::from("a")),
        ]))
        .await
        .unwrap();
    assert!(num(&kept, "Id") > 0);
    let dropped = children
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("Label", Value::from("b")),
        ]))
        .await
        .unwrap();
    assert!(num(&dropped, "Id") > 0);
    parents.add_enriched(
        Enrichment::new("Children", "Id", "ParentId", None, datasources::EnrichmentPropertyType::List),
        children.clone(),
        None,
    );
    let updated = parents
        .update(
            &record(&[
                ("Id", parent_id.clone()),
                ("Label", Value::from("p")),
                (
                    "Children",
                    encode_rows(&[
                        record(&[
                            ("Id", kept.get("Id").unwrap().clone()),
                            ("Label", Value::from("aa")),
                        ]),
                        record(&[("Label", Value::from("c"))]),
                    ]),
                ),
            ]),
            None,
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    let stored = children.read(None, None, None).await.unwrap();
    assert_eq!(stored.len(), 2);
    assert_eq!(
        stored
            .iter()
            .find(|row| row.get("Id") == kept.get("Id"))
            .unwrap()["Label"],
        Value::from("aa")
    );
    assert_eq!(
        stored
            .iter()
            .find(|row| row.get("Id") != kept.get("Id"))
            .unwrap()["Label"],
        Value::from("c")
    );
    assert!(stored
        .iter()
        .all(|row| row.get("ParentId") == Some(&parent_id)));
}

#[tokio::test]
async fn cascade_update_deletes_children_and_recreates() {
    let connection = setup(&mut []).await;
    let mut parents = mapped_crud(connection.clone(), "Parent").with_write(Arc::new(
        DefaultWriteEnrichmentStrategy::new(
            None,
            Some(Arc::new(CascadeDeleteUpdateEnrichmentStrategy)),
            None,
            false,
        ),
    ));
    let mut children_repo = mapped_crud(connection.clone(), "Child");
    let notes = Arc::new(mapped_crud(connection, "Note"));
    let parent = parents
        .create(&mut record(&[("Label", Value::from("p"))]))
        .await
        .unwrap();
    assert!(num(&parent, "Id") > 0);
    let parent_id = parent.get("Id").unwrap().clone();
    let child = children_repo
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("Label", Value::from("a")),
        ]))
        .await
        .unwrap();
    assert!(num(&child, "Id") > 0);
    let other = children_repo
        .create(&mut record(&[("Label", Value::from("other"))]))
        .await
        .unwrap();
    assert!(num(&other, "Id") > 0);
    let child_id = child.get("Id").unwrap().clone();
    let note = notes
        .create(&mut record(&[
            ("ChildId", child_id.clone()),
            ("Text", Value::from("n")),
        ]))
        .await
        .unwrap();
    assert!(num(&note, "Id") > 0);
    children_repo.add_enriched(
        Enrichment::new("Notes", "Id", "ChildId", None, datasources::EnrichmentPropertyType::List),
        notes.clone(),
        None,
    );
    let children = Arc::new(children_repo);
    parents.add_enriched(
        Enrichment::new("Children", "Id", "ParentId", None, datasources::EnrichmentPropertyType::List),
        children.clone(),
        None,
    );
    let updated = parents
        .update(
            &record(&[
                ("Id", parent_id.clone()),
                ("Label", Value::from("p")),
                (
                    "Children",
                    encode_rows(&[record(&[
                        ("Id", child_id.clone()),
                        ("Label", Value::from("fresh")),
                    ])]),
                ),
            ]),
            None,
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    let stored = children.read(None, None, None).await.unwrap();
    let created = stored
        .iter()
        .find(|row| row.get("Label") == Some(&Value::from("fresh")))
        .unwrap();
    assert_eq!(stored.len(), 2);
    assert_eq!(created.get("Id"), Some(&child_id));
    assert_eq!(created.get("Label"), Some(&Value::from("fresh")));
    assert_eq!(created.get("ParentId"), Some(&parent_id));
    assert!(notes.read(None, None, None).await.unwrap().is_empty());
}

#[tokio::test]
async fn cascade_update_replaces_links_and_keeps_related() {
    let connection = setup(&mut []).await;
    let mut parents = mapped_crud(connection.clone(), "Parent").with_write(Arc::new(
        DefaultWriteEnrichmentStrategy::new(
            None,
            Some(Arc::new(CascadeDeleteUpdateEnrichmentStrategy)),
            None,
            false,
        ),
    ));
    let tags = Arc::new(mapped_crud(connection.clone(), "Tag"));
    let links = Arc::new(mapped_crud(connection, "ParentTag"));
    let parent = parents
        .create(&mut record(&[("Label", Value::from("p"))]))
        .await
        .unwrap();
    assert!(num(&parent, "Id") > 0);
    let parent_id = parent.get("Id").unwrap().clone();
    let keep = tags
        .create(&mut record(&[("Name", Value::from("keep"))]))
        .await
        .unwrap();
    assert!(num(&keep, "Id") > 0);
    let drop = tags
        .create(&mut record(&[("Name", Value::from("drop"))]))
        .await
        .unwrap();
    assert!(num(&drop, "Id") > 0);
    let keep_id = keep.get("Id").unwrap().clone();
    let drop_id = drop.get("Id").unwrap().clone();
    let old_link = links
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("TagId", keep_id.clone()),
        ]))
        .await
        .unwrap();
    assert!(num(&old_link, "Id") > 0);
    let dropped_link = links
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("TagId", drop_id),
        ]))
        .await
        .unwrap();
    assert!(num(&dropped_link, "Id") > 0);
    let leftover = links
        .create(&mut record(&[("TagId", keep_id.clone())]))
        .await
        .unwrap();
    assert!(num(&leftover, "Id") > 0);
    let old_link_id = old_link.get("Id").unwrap().clone();
    parents.add_linked(
        Enrichment::new("Tags", "Id", "Id", None, datasources::EnrichmentPropertyType::List)
            .linked(Association::new("ParentId", "TagId")),
        tags.clone(),
        links.clone(),
        None,
    );
    let updated = parents
        .update(
            &record(&[
                ("Id", parent_id.clone()),
                ("Label", Value::from("p")),
                ("Tags", encode_rows(&[record(&[("Id", keep_id.clone())])])),
            ]),
            None,
        )
        .await
        .unwrap();
    assert_eq!(updated.rows_affected, 1);
    let stored_links: Vec<_> = links
        .read(None, None, None)
        .await
        .unwrap()
        .into_iter()
        .filter(|row| row.get("ParentId") == Some(&parent_id))
        .collect();
    assert_eq!(stored_links.len(), 1);
    assert_eq!(stored_links[0].get("TagId"), Some(&keep_id));
    assert_eq!(stored_links[0].get("ParentId"), Some(&parent_id));
    assert_ne!(stored_links[0].get("Id"), Some(&old_link_id));
    assert_eq!(tags.read(None, None, None).await.unwrap().len(), 2);
}

#[tokio::test]
async fn bulk_update_keeps_earlier_write_when_transaction_is_off() {
    let connection = setup(&mut []).await;
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    let parents = BulkCRUDRepository::new(connection, options).unwrap();
    let created = parents
        .create_bulk(&mut [
            record(&[("label", Value::from("a"))]),
            record(&[("label", Value::from("b"))]),
        ])
        .await
        .unwrap();
    assert_eq!(created.rows_affected, 2);
    let stored = parents.read(None, None, None).await.unwrap();
    let last_id = stored.iter().map(|row| num(row, "id")).max().unwrap();
    assert_eq!(created.result, Value::Int(last_id));
    let first_id = stored
        .iter()
        .find(|row| row.get("label") == Some(&Value::from("a")))
        .unwrap()
        .get("id")
        .unwrap()
        .clone();
    let error = parents
        .update_bulk(
            &[
                record(&[("id", first_id.clone()), ("label", Value::from("changed"))]),
                record(&[("label", Value::from("nope"))]),
            ],
            None,
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("Missing Id"));
    let after = parents.read(None, None, None).await.unwrap();
    assert_eq!(
        after
            .iter()
            .find(|row| row.get("id") == Some(&first_id))
            .unwrap()
            .get("label"),
        Some(&Value::from("changed"))
    );
}

#[tokio::test]
async fn bulk_update_rolls_back_when_transaction_is_on() {
    let connection = setup(&mut []).await;
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    options.write = Some(Arc::new(DefaultWriteEnrichmentStrategy::new(
        None, None, None, true,
    )));
    let parents = BulkCRUDRepository::new(connection, options).unwrap();
    let created = parents
        .create_bulk(&mut [
            record(&[("label", Value::from("a"))]),
            record(&[("label", Value::from("b"))]),
        ])
        .await
        .unwrap();
    assert_eq!(created.rows_affected, 2);
    let stored = parents.read(None, None, None).await.unwrap();
    let last_id = stored.iter().map(|row| num(row, "id")).max().unwrap();
    assert_eq!(created.result, Value::Int(last_id));
    let first_id = stored
        .iter()
        .find(|row| row.get("label") == Some(&Value::from("a")))
        .unwrap()
        .get("id")
        .unwrap()
        .clone();
    let error = parents
        .update_bulk(
            &[
                record(&[("id", first_id.clone()), ("label", Value::from("changed"))]),
                record(&[("label", Value::from("nope"))]),
            ],
            None,
        )
        .await
        .unwrap_err();
    assert!(error.to_string().contains("Missing Id"));
    let after = parents.read(None, None, None).await.unwrap();
    assert_eq!(
        after
            .iter()
            .find(|row| row.get("id") == Some(&first_id))
            .unwrap()
            .get("label"),
        Some(&Value::from("a"))
    );
}

#[tokio::test]
async fn deep_delete_cascades_children_then_parent() {
    let connection = setup(&mut []).await;
    let parents = mapped_crud(connection.clone(), "Parent");
    let mut children_repo = mapped_crud(connection.clone(), "Child");
    let notes = Arc::new(mapped_crud(connection, "Note"));
    let parent = parents
        .create(&mut record(&[("Label", Value::from("p"))]))
        .await
        .unwrap();
    assert!(num(&parent, "Id") > 0);
    let parent_id = parent.get("Id").unwrap().clone();
    let child = children_repo
        .create(&mut record(&[
            ("ParentId", parent_id.clone()),
            ("Label", Value::from("a")),
        ]))
        .await
        .unwrap();
    assert!(num(&child, "Id") > 0);
    let child_id = child.get("Id").unwrap().clone();
    let note = notes
        .create(&mut record(&[
            ("ChildId", child_id),
            ("Text", Value::from("n")),
        ]))
        .await
        .unwrap();
    assert!(num(&note, "Id") > 0);
    let other = notes
        .create(&mut record(&[
            ("Text", Value::from("other")),
            ("Label", Value::from("keep")),
        ]))
        .await
        .unwrap();
    assert!(num(&other, "Id") > 0);
    children_repo.add_enriched(
        Enrichment::new("Notes", "Id", "ChildId", None, datasources::EnrichmentPropertyType::List),
        notes.clone(),
        None,
    );
    let children = Arc::new(children_repo);
    let mut parents = parents;
    parents.add_enriched(
        Enrichment::new("Children", "Id", "ParentId", None, datasources::EnrichmentPropertyType::List),
        children.clone(),
        None,
    );
    let removed = parents
        .delete(&record(&[("Id", parent_id)]), None)
        .await
        .unwrap();
    assert_eq!(removed.rows_affected, 1);
    assert!(parents.read(None, None, None).await.unwrap().is_empty());
    assert!(children.read(None, None, None).await.unwrap().is_empty());
    assert_eq!(
        notes.read(None, None, None).await.unwrap()[0]["Text"],
        Value::from("other")
    );
}

#[tokio::test]
async fn deep_many_readonly_via_throws() {
    let connection = setup(&mut []).await;
    let mut parents = mapped_crud(connection.clone(), "Parent");
    parents.add_enriched(
        Enrichment::new("Children", "Id", "ParentId", None, datasources::EnrichmentPropertyType::List),
        Arc::new(mapped(connection, "Child")),
        None,
    );
    let error = parents
        .create(&mut record(&[(
            "Children",
            encode_rows(&[record(&[("Label", Value::from("a"))])]),
        )]))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("Children"));
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
    parents.add_enriched(
        Enrichment::new("children", "id", "parent_id", None, datasources::EnrichmentPropertyType::List),
        children.clone(),
        None,
    );
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
    let stored_parents = parents.read(None, None, None).await.unwrap();
    assert_eq!(stored_parents.len(), 2);
    assert_eq!(children.read(None, None, None).await.unwrap().len(), 3);
    assert_eq!(response.rows_affected, 2);
    let last_id = stored_parents
        .iter()
        .map(|row| num(row, "id"))
        .max()
        .unwrap();
    assert_eq!(response.result, Value::Int(last_id));
}
