use crate::command::SqliteCommand;
use crate::connection::SqliteConnection;
use datasources::{
    Command, Connection, CRUDRepository, CRUDRepositoryOptions, Enrichment, EnrichmentPropertyType,
    Native, Schema, SchemaColumn, SchemaObject, Value,
};
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct AddressType {
    id: i64,
    name: String,
}

#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct AddressView {
    id: i64,
    contact_id: String,
    line1: String,
    line2: Option<String>,
    latitude: Option<i32>,
    address_type_name: String,
}

impl Default for AddressView {
    fn default() -> Self {
        Self {
            id: 0,
            contact_id: String::new(),
            line1: String::new(),
            line2: None,
            latitude: None,
            address_type_name: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct LocalePref {
    locale: String,
    timezone: String,
}

#[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct PrefRow {
    id: i64,
    contact_id: i64,
    key: String,
    locale: String,
    timezone: String,
}

#[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
struct ContactView {
    id: i64,
    name: String,
    addresses: Vec<AddressView>,
    prefs: HashMap<String, LocalePref>,
}

impl SchemaObject for AddressType {}
impl SchemaObject for AddressView {}
impl SchemaObject for LocalePref {}
impl SchemaObject for PrefRow {}
impl SchemaObject for ContactView {}

fn column(type_name: &str, nullable: bool, has_default: bool, is_id: bool) -> SchemaColumn {
    SchemaColumn {
        type_name: type_name.into(),
        nullable,
        has_default,
        is_id,
    }
}

async fn open() -> Arc<SqliteConnection> {
    let conn = Arc::new(SqliteConnection::new(Some(":memory:".into())));
    conn.open().await.unwrap();
    let command = SqliteCommand::new(Arc::clone(&conn));
    for sql in [
        "CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        "CREATE TABLE address_types (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
        "CREATE TABLE addresses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contact_id INTEGER NOT NULL,
            address_type_id INTEGER NOT NULL,
            line1 TEXT NOT NULL,
            line2 TEXT,
            latitude INTEGER
        )",
        "CREATE TABLE prefs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contact_id INTEGER NOT NULL,
            key TEXT NOT NULL,
            locale TEXT NOT NULL,
            timezone TEXT NOT NULL
        )",
        "INSERT INTO address_types (name) VALUES ('Home')",
        "INSERT INTO address_types (name) VALUES ('Work')",
    ] {
        command.execute_native(&Native::new(sql), None).await.unwrap();
    }
    conn
}

fn contact_repo(connection: Arc<dyn Connection>) -> CRUDRepository<ContactView> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "contacts".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "name".into()];
    options.create_properties = Some(vec!["name".into()]);
    options.update_properties = Some(vec!["name".into()]);
    options.schema = Schema::new(HashMap::from([
        ("id".into(), column("integer", false, true, true)),
        ("name".into(), column("string", false, false, false)),
    ]));
    CRUDRepository::<ContactView>::new(connection, options).unwrap()
}

fn address_repo(connection: Arc<dyn Connection>) -> CRUDRepository<AddressView> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "addresses".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec![
        "id".into(),
        "contact_id".into(),
        "address_type_id".into(),
        "line1".into(),
        "line2".into(),
        "latitude".into(),
    ];
    options.create_properties = Some(vec![
        "contact_id".into(),
        "address_type_id".into(),
        "line1".into(),
        "line2".into(),
        "latitude".into(),
    ]);
    options.update_properties = Some(vec![
        "contact_id".into(),
        "address_type_id".into(),
        "line1".into(),
        "line2".into(),
        "latitude".into(),
    ]);
    options.schema = Schema::new(HashMap::from([
        ("id".into(), column("integer", false, true, true)),
        ("contact_id".into(), column("integer", false, false, false)),
        ("address_type_id".into(), column("integer", false, false, false)),
        ("line1".into(), column("string", false, false, false)),
        ("line2".into(), column("string", true, false, false)),
        ("latitude".into(), column("integer", true, false, false)),
    ]));
    CRUDRepository::<AddressView>::new(connection, options).unwrap()
}

fn type_repo(connection: Arc<dyn Connection>) -> CRUDRepository<AddressType> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "address_types".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec!["id".into(), "name".into()];
    options.create_properties = Some(vec!["name".into()]);
    options.schema = Schema::new(HashMap::from([
        ("id".into(), column("integer", false, true, true)),
        ("name".into(), column("string", false, false, false)),
    ]));
    CRUDRepository::<AddressType>::new(connection, options).unwrap()
}

fn pref_repo(connection: Arc<dyn Connection>) -> CRUDRepository<PrefRow> {
    let mut options = CRUDRepositoryOptions::default();
    options.entity = "prefs".into();
    options.ids = vec!["id".into()];
    options.read_properties = vec![
        "id".into(),
        "contact_id".into(),
        "key".into(),
        "locale".into(),
        "timezone".into(),
    ];
    options.create_properties = Some(vec![
        "contact_id".into(),
        "key".into(),
        "locale".into(),
        "timezone".into(),
    ]);
    options.update_properties = Some(vec![
        "contact_id".into(),
        "key".into(),
        "locale".into(),
        "timezone".into(),
    ]);
    options.schema = Schema::new(HashMap::from([
        ("id".into(), column("integer", false, true, true)),
        ("contact_id".into(), column("integer", false, false, false)),
        ("key".into(), column("string", false, false, false)),
        ("locale".into(), column("string", false, false, false)),
        ("timezone".into(), column("string", false, false, false)),
    ]));
    CRUDRepository::<PrefRow>::new(connection, options).unwrap()
}

fn wired(connection: Arc<SqliteConnection>) -> CRUDRepository<ContactView> {
    let types = Arc::new(type_repo(connection.clone()));
    let mut addresses = address_repo(connection.clone());
    addresses.add_enriched(
        Enrichment::new(
            "address_type_name",
            "address_type_id",
            "id",
            Some("name".into()),
            EnrichmentPropertyType::Object,
        ),
        types,
        None,
    );
    let prefs = Arc::new(pref_repo(connection.clone()));
    let mut contacts = contact_repo(connection);
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
    contacts.add_enriched(
        Enrichment::new(
            "prefs",
            "id",
            "contact_id",
            None,
            EnrichmentPropertyType::Dictionary,
        ),
        prefs,
        None,
    );
    contacts
}

fn ada() -> ContactView {
    ContactView {
        name: "Ada".into(),
        addresses: vec![AddressView {
            address_type_name: "Home".into(),
            line1: "12 Analytical Engine".into(),
            line2: Some("Room 12".into()),
            ..AddressView::default()
        }],
        prefs: HashMap::from([(
            "home".into(),
            LocalePref {
                locale: "en-GB".into(),
                timezone: "Europe/London".into(),
            },
        )]),
        ..ContactView::default()
    }
}

#[tokio::test]
async fn create_rereads_typed_list_option_and_dictionary() {
    let conn = open().await;
    let contacts = wired(conn);
    let created = contacts.create(ada()).await.unwrap();
    assert!(created.id > 0);
    assert_eq!(created.addresses.len(), 1);
    assert_eq!(created.addresses[0].line1, "12 Analytical Engine");
    assert_eq!(created.addresses[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(created.addresses[0].address_type_name, "Home");
    assert!(created.addresses[0].latitude.is_none());
    assert_eq!(
        created.prefs.get("home").map(|pref| pref.locale.as_str()),
        Some("en-GB")
    );
    let read = contacts.read(&()).try_collect::<Vec<_>>().await.unwrap();
    assert_eq!(read[0].addresses[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(read[0].prefs["home"].timezone, "Europe/London");
}

#[tokio::test]
async fn update_stamp_keeps_typed_list_line2() {
    let conn = open().await;
    let contacts = wired(conn);
    let mut created = contacts.create(ada()).await.unwrap();
    created.addresses[0].line1 = "12 Engine".into();
    contacts.update(created.clone()).await.unwrap();
    let read = contacts
        .read_single_by_id(&Value::Int(created.id))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(read.addresses[0].line1, "12 Engine");
    assert_eq!(read.addresses[0].line2.as_deref(), Some("Room 12"));
    assert_eq!(read.addresses[0].address_type_name, "Home");
}
