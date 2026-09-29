use datasources::{
    default, ConvertSchema, Mapper, Parameter, Row, RowTypeConverter, TransformMapper, Value,
};

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

#[tokio::test]
async fn run_raw_sql() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)",
            None,
        )
        .await
        .unwrap();
    command
        .execute("INSERT INTO items (id, name) VALUES (1, 'ada')", None)
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT id, name FROM items", None)
        .await
        .unwrap();
    assert!(reader.next().await.unwrap());
    let row = reader.current().unwrap();
    assert_eq!(row.get("id"), Some(&Value::Int(1)));
    assert_eq!(row.get("name"), Some(&Value::from("ada")));
    assert!(!reader.next().await.unwrap());
}

#[tokio::test]
async fn run_execute_reader_and_return_name_value() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute("CREATE TABLE settings (name TEXT, value TEXT)", None)
        .await
        .unwrap();
    let parameters = [
        Parameter::new("name", "theme"),
        Parameter::new("value", "dark"),
    ];
    command
        .execute(
            "INSERT INTO settings (name, value) VALUES ($name, $value)",
            Some(&parameters),
        )
        .await
        .unwrap();
    let reader = command
        .execute_reader("SELECT name, value FROM settings", None)
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<NameValue>();
    let mut items = Vec::new();
    while reader.next().await.unwrap() {
        items.push(converter.convert(&reader.current().unwrap()).unwrap());
    }
    assert_eq!(items[0].get("name"), Some(&Value::from("theme")));
    assert_eq!(items[0].get("value"), Some(&Value::from("dark")));
}

#[tokio::test]
async fn execute_insert_update_and_delete() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT)",
            None,
        )
        .await
        .unwrap();
    command
        .execute(
            "INSERT INTO items (id, name) VALUES ($id, $name)",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
        )
        .await
        .unwrap();
    command
        .execute(
            "UPDATE items SET name = $name WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
        )
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<Item>();
    let updated = {
        let reader = command
            .execute_reader(
                "SELECT id, name FROM items WHERE id = $id",
                Some(&[Parameter::new("id", 1_i64)]),
            )
            .await
            .unwrap();
        let row = if reader.next().await.unwrap() {
            Some(converter.convert(&reader.current().unwrap()).unwrap())
        } else {
            None
        };
        row
    };
    command
        .execute(
            "DELETE FROM items WHERE id = $id",
            Some(&[Parameter::new("id", 1_i64)]),
        )
        .await
        .unwrap();
    let gone = {
        let reader = command
            .execute_reader(
                "SELECT id, name FROM items WHERE id = $id",
                Some(&[Parameter::new("id", 1_i64)]),
            )
            .await
            .unwrap();
        let row = if reader.next().await.unwrap() {
            Some(converter.convert(&reader.current().unwrap()).unwrap())
        } else {
            None
        };
        row
    };
    let updated = updated.unwrap();
    assert_eq!(updated.get("id"), Some(&Value::Int(1)));
    assert_eq!(updated.get("name"), Some(&Value::from("A")));
    assert!(gone.is_none());
}

#[tokio::test]
async fn execute_reader_maps_flat_contacts() {
    let _guard = super::env_lock();
    let ds = default().unwrap();
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    command
        .execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT,
                email TEXT,
                contact_name1 TEXT,
                contact_name2 TEXT
            )",
            None,
        )
        .await
        .unwrap();
    command
        .execute(
            "INSERT INTO users (id, username, email, contact_name1, contact_name2)
             VALUES (1, 'bobby', 'bob@asdf.com', 'bob', 'roberta')",
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
    let mapper = user_contacts_mapper();
    let mut user = None;
    while reader.next().await.unwrap() {
        user = Some(mapper.map(converter.convert(&reader.current().unwrap()).unwrap()));
    }
    let user = user.unwrap();
    assert_eq!(user.username, "bobby");
    assert_eq!(user.email, "bob@asdf.com");
    assert_eq!(
        user.contacts
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
    let connection = ds.create_connection().await.unwrap();
    connection.open().await.unwrap();
    let command = connection.create_command();
    let reader = command
        .execute_reader("SELECT 1 AS id; SELECT 2 AS id", None)
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
