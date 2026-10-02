use datasources::{
    default, Casing, ChangeCasingMapper, ConvertSchema, FlatToNestedMapper,
    MappedRow, MappedValue,
    Mapper, Native, Parameter, Row, RowTypeConverter, TransformMapper, TypeMap, TypeMapField,
    TypeMapGroup, Value,
};
use datasources_sql_specs::apply_schema;
use futures_util::TryStreamExt;

struct NameValue;

impl ConvertSchema for NameValue {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT"), ("value", "TEXT")]
    }
}

struct Item;

impl ConvertSchema for Item {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("id", "INTEGER"), ("name", "TEXT")]
    }
}

struct Contact {
    name: String,
}

struct User {
    id: i64,
    username: String,
    email: String,
    contact_name1: String,
    contact_name2: String,
    contacts: Vec<Contact>,
}

impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("username", "TEXT"),
            ("email", "TEXT"),
            ("contact_name1", "TEXT"),
            ("contact_name2", "TEXT"),
        ]
    }
}

#[tokio::test]
async fn reads_boolean_as_bool_and_leaves_integer() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    apply_schema(connection.as_ref(), "flags").await.unwrap();
    let command = connection.create_command();
    command
        .execute_native(&Native::new("INSERT INTO flags (enabled, count) VALUES (1, 1)"), None)
        .await
        .unwrap();
    let reader = command
        .execute_reader_native(
            &Native::new("SELECT enabled, count, 1 AS one FROM flags"),
            None,
        )
        .await
        .unwrap();
    assert!(reader.next().await.unwrap());
    let row = reader.current().unwrap();
    assert_eq!(row.get("enabled"), Some(&Value::Bool(true)));
    assert_eq!(row.get("count"), Some(&Value::Int(1)));
    assert_eq!(row.get("one"), Some(&Value::Int(1)));
}

#[tokio::test]
async fn run_execute_reader_on_a_simple_list() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    apply_schema(ds.create_connection().await.unwrap().as_ref(), "items").await.unwrap();
    command
        .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (1, 'ada')"), None)
        .await
        .unwrap();
    command
        .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (2, 'grace')"), None)
        .await
        .unwrap();
    command
        .execute_native(&Native::new("INSERT INTO items (id, name) VALUES (3, 'bea')"), None)
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT id, name FROM items", None)
        .await
        .unwrap();
    let rows: Vec<_> = reader.read().try_collect().await.unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].get("id"), Some(&Value::Int(1)));
    assert_eq!(rows[0].get("name"), Some(&Value::from("ada")));
    assert_eq!(rows[1].get("id"), Some(&Value::Int(2)));
    assert_eq!(rows[1].get("name"), Some(&Value::from("grace")));
    assert_eq!(rows[2].get("id"), Some(&Value::Int(3)));
    assert_eq!(rows[2].get("name"), Some(&Value::from("bea")));
}

#[tokio::test]
async fn read_and_apply_case_mappings() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    apply_schema(ds.create_connection().await.unwrap().as_ref(), "case-contacts").await.unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO contacts (user_id, first_name) VALUES (1, 'Ada')"),
            None,
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT user_id, first_name FROM contacts", None)
        .await
        .unwrap();
    let casing = ChangeCasingMapper::new(Casing::Pascal, Casing::Snake);
    let mapped: Vec<Row> = reader
        .read()
        .map_ok(|source| {
            source
                .into_iter()
                .map(|(key, value)| (Mapper::reverse_map(&casing, key), value))
                .collect()
        })
        .try_collect()
        .await
        .unwrap();
    assert_eq!(mapped[0].get("UserId"), Some(&Value::Int(1)));
    assert_eq!(mapped[0].get("FirstName"), Some(&Value::from("Ada")));
}

#[tokio::test]
async fn execute_reader_then_apply_flat_to_nested_mapper() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    apply_schema(ds.create_connection().await.unwrap().as_ref(), "flat-users").await.unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'bob')"),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO users (id, username, contact_name) VALUES (1, 'bobby', 'roberta')"),
            None,
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT id, username, contact_name FROM users", None)
        .await
        .unwrap();
    let rows: Vec<_> = reader.read().try_collect().await.unwrap();
    let mapper = FlatToNestedMapper::new(TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("username".into(), TypeMapField::column("username")),
            (
                "contacts".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "contact_name",
                    vec![("name".into(), TypeMapField::column("contact_name"))],
                )),
            ),
        ],
    ));
    let nested = Mapper::map(&mapper, rows);
    assert_eq!(nested.len(), 1);
    assert_eq!(nested[0].get("id"), Some(&MappedValue::Cell(Value::Int(1))));
    assert_eq!(
        nested[0].get("username"),
        Some(&MappedValue::Cell(Value::from("bobby")))
    );
    assert_eq!(
        nested[0].get("contacts"),
        Some(&MappedValue::Many(vec![
            MappedRow::from([("name".into(), MappedValue::Cell(Value::from("bob")))]),
            MappedRow::from([("name".into(), MappedValue::Cell(Value::from("roberta")),)]),
        ]))
    );
}

#[tokio::test]
async fn run_execute_reader_and_return_name_values() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    apply_schema(connection.as_ref(), "settings").await.unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO settings (name, value) VALUES ('theme', 'dark')"),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO settings (name, value) VALUES ('lang', 'en')"),
            None,
        )
        .await
        .unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO settings (name, value) VALUES ('size', 'lg')"),
            None,
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT name, value FROM settings", None)
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<NameValue>();
    let items: Vec<_> = reader
        .read()
        .map_ok(|row| converter.convert(&row).unwrap())
        .try_collect()
        .await
        .unwrap();
    assert_eq!(items[0].get("name"), Some(&Value::from("theme")));
    assert_eq!(items[0].get("value"), Some(&Value::from("dark")));
    assert_eq!(items[1].get("name"), Some(&Value::from("lang")));
    assert_eq!(items[1].get("value"), Some(&Value::from("en")));
    assert_eq!(items[2].get("name"), Some(&Value::from("size")));
    assert_eq!(items[2].get("value"), Some(&Value::from("lg")));
}

#[tokio::test]
async fn execute_insert_read_update_and_delete() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    apply_schema(connection.as_ref(), "items").await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "INSERT INTO items (id, name) VALUES ($id, $name)",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
        )
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<Item>();
    let created_reader = command
        .execute_reader(
            "SELECT id, name FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let created: Vec<_> = created_reader
        .read()
        .map_ok(|row| converter.convert(&row).unwrap())
        .try_collect()
        .await
        .unwrap();
    drop(created_reader);
    let response = command
        .execute(
            "UPDATE items SET name = $name WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
        )
        .await
        .unwrap();
    let updated_reader = command
        .execute_reader(
            "SELECT id, name FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let updated: Vec<_> = updated_reader
        .read()
        .map_ok(|row| converter.convert(&row).unwrap())
        .try_collect()
        .await
        .unwrap();
    drop(updated_reader);
    command
        .execute(
            "DELETE FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let gone_reader = command
        .execute_reader(
            "SELECT id, name FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let gone: Vec<_> = gone_reader
        .read()
        .map_ok(|row| converter.convert(&row).unwrap())
        .try_collect()
        .await
        .unwrap();
    assert_eq!(created[0].get("id"), Some(&Value::Int(1)));
    assert_eq!(created[0].get("name"), Some(&Value::from("a")));
    assert_eq!(response.rows_affected, 1);
    assert_eq!(updated[0].get("id"), Some(&Value::Int(1)));
    assert_eq!(updated[0].get("name"), Some(&Value::from("A")));
    assert!(gone.is_empty());
}

#[tokio::test]
async fn execute_reader_maps_flat_contacts() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    apply_schema(
        ds.create_connection().await.unwrap().as_ref(),
        "flat-contact-users",
    )
    .await
    .unwrap();
    command
        .execute_native(
            &Native::new("INSERT INTO users (id, username, email, contact_name1, contact_name2)
             VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')"),
            None,
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader(
            "SELECT contact_name1, contact_name2, email, id, username FROM users",
            None,
        )
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<User>();
    fn user_contacts_mapper() -> TransformMapper<Row, User> {
        fn text_of(row: &Row, key: &str) -> String {
            match row.get(key) {
                Some(Value::Text(text)) => text.clone(),
                _ => String::new(),
            }
        }
        TransformMapper::new(
            |row| {
                let contact_name1 = text_of(&row, "contact_name1");
                let contact_name2 = text_of(&row, "contact_name2");
                User {
                    id: match row.get("id") {
                        Some(Value::Int(id)) => *id,
                        _ => 0,
                    },
                    username: text_of(&row, "username"),
                    email: text_of(&row, "email"),
                    contacts: vec![
                        Contact {
                            name: contact_name1.clone(),
                        },
                        Contact {
                            name: contact_name2.clone(),
                        },
                    ],
                    contact_name1,
                    contact_name2,
                }
            },
            |mut user| {
                user.contact_name1 = user
                    .contacts
                    .first()
                    .map(|contact| contact.name.clone())
                    .unwrap_or_default();
                user.contact_name2 = user
                    .contacts
                    .get(1)
                    .map(|contact| contact.name.clone())
                    .unwrap_or_default();
                let mut row = Row::new();
                row.insert("id".into(), Value::Int(user.id));
                row.insert("username".into(), Value::from(user.username));
                row.insert("email".into(), Value::from(user.email));
                row.insert("contact_name1".into(), Value::from(user.contact_name1));
                row.insert("contact_name2".into(), Value::from(user.contact_name2));
                row
            },
        )
    }
    let mapper = user_contacts_mapper();
    let users: Vec<_> = reader
        .read()
        .map_ok(|row| mapper.map(converter.convert(&row).unwrap()))
        .try_collect()
        .await
        .unwrap();
    assert_eq!(users[0].username, "bobby");
    assert_eq!(users[0].email, "bob@asdf.com");
    assert_eq!(
        users[0]
            .contacts
            .iter()
            .map(|contact| contact.name.as_str())
            .collect::<Vec<_>>(),
        ["bob", "roberta"]
    );
}

#[tokio::test]
async fn read_multiple_results() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let command = ds.create_command(None).await.unwrap();
    let reader = command
        .execute_reader_native(&Native::new("SELECT 1 AS id; SELECT 2 AS id"), None)
        .await
        .unwrap();
    let mut first = Vec::new();
    while reader.next().await.unwrap() {
        first.push(reader.current());
    }
    let moved = reader.next_result().await.unwrap();
    let mut second = Vec::new();
    while reader.next().await.unwrap() {
        second.push(reader.current());
    }
    let again = reader.next_result().await.unwrap();
    assert!(moved);
    assert!(!again);
    assert_eq!(first[0].as_ref().unwrap().get("id"), Some(&Value::Int(1)));
    assert_eq!(second[0].as_ref().unwrap().get("id"), Some(&Value::Int(2)));
}
