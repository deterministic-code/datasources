use crate::{
    Command, ConvertSchema, Mapper, MockCommand, Parameter, Row, RowTypeConverter, TransformMapper,
    Value,
};

fn row(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(key, value)| ((*key).to_string(), value.clone()))
        .collect()
}

struct NameValue;

impl ConvertSchema for NameValue {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![("name", "TEXT"), ("value", "TEXT")]
    }
}

struct UserRow;

impl ConvertSchema for UserRow {
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

fn user_contacts_mapper() -> TransformMapper<Row, Row> {
    TransformMapper::new(
        |mut row: Row| {
            let first = row.get("contact_name1").cloned().unwrap_or(Value::Null);
            let second = row.get("contact_name2").cloned().unwrap_or(Value::Null);
            row.insert(
                "contacts".into(),
                Value::from(format!(
                    "{},{}",
                    match first {
                        Value::Text(text) => text,
                        _ => String::new(),
                    },
                    match second {
                        Value::Text(text) => text,
                        _ => String::new(),
                    }
                )),
            );
            row
        },
        |row| row,
    )
}

#[tokio::test]
async fn execute_reader_returns_name_value_objects() {
    let command = MockCommand::new();
    command.reader_rows.lock().unwrap().push(row(&[
        ("name", Value::from("theme")),
        ("value", Value::from("dark")),
    ]));
    let reader = command
        .execute_reader("SELECT name, value FROM settings", None)
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<NameValue>();
    let mut items = Vec::new();
    while reader.next().await.unwrap() {
        items.push(converter.convert(&reader.current().unwrap()).unwrap());
    }
    assert_eq!(items[0]["name"], Value::from("theme"));
    assert_eq!(items[0]["value"], Value::from("dark"));
}

#[tokio::test]
async fn execute_insert_update_delete() {
    let command = MockCommand::new();
    command
        .execute(
            "insert",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "a")]),
        )
        .await
        .unwrap();
    command
        .execute(
            "update",
            Some(&[Parameter::new("id", 1_i64), Parameter::new("name", "A")]),
        )
        .await
        .unwrap();
    command
        .execute("delete", Some(&[Parameter::new("id", 1_i64)]))
        .await
        .unwrap();
    assert_eq!(command.execute_logs.lock().unwrap()[0].0, "insert");
    assert_eq!(command.execute_logs.lock().unwrap()[1].0, "update");
    assert_eq!(command.execute_logs.lock().unwrap()[2].0, "delete");
    assert_eq!(
        command.execute_logs.lock().unwrap()[0]
            .1
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["id", "name"]
    );
}

#[tokio::test]
async fn execute_reader_maps_flat_contacts() {
    let command = MockCommand::new();
    command.reader_rows.lock().unwrap().push(row(&[
        ("id", Value::Int(1)),
        ("username", Value::from("bobby")),
        ("email", Value::from("bob@asdf.com")),
        ("contact_name1", Value::from("bob")),
        ("contact_name2", Value::from("roberta")),
    ]));
    let reader = command
        .execute_reader(
            "SELECT contact_name1, contact_name2, email, id, username FROM users",
            None,
        )
        .await
        .unwrap();
    let converter = RowTypeConverter::of::<UserRow>();
    let mapper = user_contacts_mapper();
    let mut user = None;
    while reader.next().await.unwrap() {
        user = Some(Mapper::map(
            &mapper,
            converter.convert(&reader.current().unwrap()).unwrap(),
        ));
    }
    assert_eq!(user.as_ref().unwrap()["username"], Value::from("bobby"));
    assert_eq!(user.as_ref().unwrap()["email"], Value::from("bob@asdf.com"));
    assert_eq!(
        user.as_ref().unwrap()["contacts"],
        Value::from("bob,roberta")
    );
}
