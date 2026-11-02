use datasources::*;
use datasources_memory::MemoryDriver;
use futures_util::TryStreamExt;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Parent {
    id: i64,
    label: String,
    children: Vec<Child>,
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Child {
    id: i64,
    parent_id: i64,
    label: String,
    line2: Option<String>,
    created: Option<chrono::DateTime<chrono::Utc>>,
}

impl SchemaObject for Parent {}
impl SchemaObject for Child {}

fn parent_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Parent".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "label".into()];
    options.create_properties = Some(vec!["label".into()]);
    options.schema = Schema::new(std::collections::HashMap::from([
        ("id".into(), SchemaColumn {
            type_name: "integer".into(),
            nullable: false,
            has_default: true,
            is_id: true,
        }),
        ("label".into(), SchemaColumn {
            type_name: "string".into(),
            nullable: false,
            has_default: false,
            is_id: false,
        }),
    ]));
    options
}

fn child_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Child".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "parent_id".into(), "label".into(), "line2".into(), "created".into()];
    options.create_properties = Some(vec!["parent_id".into(), "label".into(), "line2".into(), "created".into()]);
    options
}

fn child_repo(connection: Arc<dyn Connection>) -> Arc<CRUDRepository> {
    Arc::new(CRUDRepository::<Row>::new(connection, child_options()).unwrap())
}

fn child_typed_repo(connection: Arc<dyn Connection>) -> Arc<CRUDRepository<Child>> {
    Arc::new(CRUDRepository::<Child>::new(connection, child_options()).unwrap())
}

#[test]
fn typed_parent_payload_reads_children() {
    let parent = Parent {
        label: "p".into(),
        children: vec![Child {
            label: "a".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let rows = parent.payload("children");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get("label"), Some(&Value::from("a")));
    let mut empty = Parent::default();
    empty.set_payload("children", rows).unwrap();
    assert_eq!(empty.children[0].label, "a");
}

#[tokio::test]
async fn typed_create_persists_and_reads_list_children() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut parents = CRUDRepository::<Parent>::new(Arc::clone(&connection), parent_options()).unwrap();
    let children = child_repo(connection);
    parents.add_enriched(
        Enrichment::new(
            "children",
            "id",
            "parent_id",
            None,
            EnrichmentPropertyType::List,
        ),
        children.clone(),
        None,
    );
    let stamp = chrono::DateTime::parse_from_rfc3339("2024-06-01T12:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let created = parents
        .create(Parent {
            label: "p".into(),
            children: vec![Child {
                label: "a".into(),
                line2: Some("Room 12".into()),
                created: Some(stamp),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    assert_eq!(created.children.len(), 1);
    assert_eq!(created.children[0].label, "a");
    assert_eq!(created.children[0].parent_id, created.id);
    assert_eq!(created.children[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(created.children[0].created, Some(stamp));
    let read = parents.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].children[0].label, "a");
    assert_eq!(read[0].children[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(read[0].children[0].created, Some(stamp));
}

#[tokio::test]
async fn typed_create_persists_list_children_on_typed_repo() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut parents = CRUDRepository::<Parent>::new(Arc::clone(&connection), parent_options()).unwrap();
    let children = child_typed_repo(connection);
    parents.add_enriched(
        Enrichment::new(
            "children",
            "id",
            "parent_id",
            None,
            EnrichmentPropertyType::List,
        ),
        children.clone(),
        None,
    );
    let stamp = chrono::DateTime::parse_from_rfc3339("2024-06-01T12:00:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    let created = parents
        .create(Parent {
            label: "p".into(),
            children: vec![Child {
                label: "a".into(),
                line2: Some("Room 12".into()),
                created: Some(stamp),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    assert_eq!(created.children.len(), 1);
    assert_eq!(created.children[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(created.children[0].parent_id, created.id);
    let read = parents.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read[0].children[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(read[0].children[0].created, Some(stamp));
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Person {
    id: i64,
    gender_type_name: String,
}

impl SchemaObject for Person {}

fn person_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Person".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "gender_type_id".into()];
    options.create_properties = Some(vec!["gender_type_id".into()]);
    options.schema = Schema::new(std::collections::HashMap::from([
        ("id".into(), SchemaColumn {
            type_name: "integer".into(),
            nullable: false,
            has_default: true,
            is_id: true,
        }),
        ("gender_type_id".into(), SchemaColumn {
            type_name: "integer".into(),
            nullable: false,
            has_default: false,
            is_id: false,
        }),
    ]));
    options
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct Gender {
    id: i64,
    name: String,
}

impl SchemaObject for Gender {}

fn gender_options() -> CRUDRepositoryOptions {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "Gender".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "name".into()];
    options.create_properties = Some(vec!["name".into()]);
    options
}

fn gender_repo(connection: Arc<dyn Connection>) -> Arc<CRUDRepository> {
    Arc::new(CRUDRepository::<Row>::new(connection, gender_options()).unwrap())
}

fn gender_typed_repo(connection: Arc<dyn Connection>) -> Arc<CRUDRepository<Gender>> {
    Arc::new(CRUDRepository::<Gender>::new(connection, gender_options()).unwrap())
}

#[tokio::test]
async fn typed_create_resolves_lookup_name_without_source_property() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let genders = gender_repo(Arc::clone(&connection));
    let mut female = Row::new();
    female.insert("name".into(), Value::from("Female"));
    let female = genders.create(female).await.unwrap();
    let mut people = CRUDRepository::<Person>::new(connection, person_options()).unwrap();
    people.add_enriched(
        Enrichment::new(
            "gender_type_name",
            "gender_type_id",
            "id",
            Some("name".into()),
            EnrichmentPropertyType::Object,
        ),
        genders.clone(),
        None,
    );
    let created = people
        .create(Person {
            gender_type_name: "Female".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    assert_eq!(created.gender_type_name, "Female");
    assert_eq!(female.get("name"), Some(&Value::from("Female")));
    let read = people.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].gender_type_name, "Female");
    let unknown = people
        .create(Person {
            gender_type_name: "Nope".into(),
            ..Default::default()
        })
        .await
        .err()
        .unwrap();
    assert!(unknown.to_string().contains("Unknown gender_type_name"));
}

#[tokio::test]
async fn typed_create_resolves_lookup_on_typed_gender_repo() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let genders = gender_typed_repo(Arc::clone(&connection));
    let female = genders
        .create(Gender {
            name: "Female".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    let mut people = CRUDRepository::<Person>::new(connection, person_options()).unwrap();
    people.add_enriched(
        Enrichment::new(
            "gender_type_name",
            "gender_type_id",
            "id",
            Some("name".into()),
            EnrichmentPropertyType::Object,
        ),
        genders.clone(),
        None,
    );
    let created = people
        .create(Person {
            gender_type_name: "Female".into(),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    assert_eq!(created.gender_type_name, "Female");
    assert_eq!(female.name, "Female");
    let read = people.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read[0].gender_type_name, "Female");
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct AddressView {
    id: i64,
    contact_id: String,
    line: String,
}

#[derive(Clone, Debug, Default, PartialEq, Facet)]
#[facet(crate = ::datasources::facet)]
struct ContactView {
    id: i64,
    addresses: Vec<AddressView>,
}

impl SchemaObject for AddressView {}
impl SchemaObject for ContactView {}

#[tokio::test]
async fn typed_via_skips_integer_contact_id_on_string_view() {
    let connection = MemoryDriver.connection(None);
    connection.open().await.unwrap();
    let mut contacts = CRUDRepository::<ContactView>::new(Arc::clone(&connection), {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Contact".into();
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into()];
        options.create_properties = Some(vec![]);
        options.schema = Schema::new(std::collections::HashMap::from([(
            "id".into(),
            SchemaColumn {
                type_name: "integer".into(),
                nullable: false,
                has_default: true,
                is_id: true,
            },
        )]));
        options
    })
    .unwrap();
    let addresses = CRUDRepository::<AddressView>::new(connection, {
        let mut options = CRUDRepositoryOptions::default();
        options.entity = "Address".into();
        options.ids = vec!["id".into()];
        options.read_properties = vec!["id".into(), "contact_id".into(), "line".into()];
        options.create_properties = Some(vec!["contact_id".into(), "line".into()]);
        options.schema = Schema::new(std::collections::HashMap::from([
            (
                "id".into(),
                SchemaColumn {
                    type_name: "integer".into(),
                    nullable: false,
                    has_default: true,
                    is_id: true,
                },
            ),
            (
                "contact_id".into(),
                SchemaColumn {
                    type_name: "integer".into(),
                    nullable: false,
                has_default: false,
                is_id: false,
            },
            ),
            (
                "line".into(),
                SchemaColumn {
                    type_name: "string".into(),
                    nullable: false,
                has_default: false,
                is_id: false,
            },
            ),
        ]));
        options
    })
    .unwrap();
    contacts.add_enriched(
        Enrichment::new(
            "addresses",
            "id",
            "contact_id",
            None,
            EnrichmentPropertyType::List,
        ),
        Arc::new(addresses),
        None,
    );
    let mut row = Row::new();
    row.insert("id".into(), Value::Int(1));
    row.insert("contact_id".into(), Value::Int(9));
    row.insert("line".into(), Value::from("1 Main"));
    let converted = AddressView::object_converter().convert(&row).unwrap();
    assert_eq!(converted.line, "1 Main");
    assert_eq!(converted.contact_id, "");
    let created = contacts
        .create(ContactView {
            addresses: vec![AddressView {
                contact_id: String::new(),
                line: "1 Main".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(created.id > 0);
    let read = contacts.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read.len(), 1);
}
