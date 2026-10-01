use std::sync::Arc;

use crate::mappers::{DictionaryMapper, Mapper};
use crate::*;

fn mappings() -> StatementMappings {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("User".to_string(), "people".to_string()),
        ("Role".to_string(), "roles".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([
            ("Name".to_string(), "full_name".to_string()),
            ("title".to_string(), "book.title".to_string()),
            ("id".to_string(), "user_id".to_string()),
        ]));
    StatementMappings::new(entities, properties)
}

fn cell(name: &str, value: &Value) -> Result<Value> {
    Ok(match value {
        Value::Text(text) => Value::Text(format!("{name}:{}", text.to_uppercase())),
        other => other.clone(),
    })
}

#[test]
fn maps_select() {
    let where_clause = Where::new(Operator::equals("Name", "Bea"));
    let having = Where::new(Operator::equals("Name", "Bea"));
    let fields = [
        "Name",
        "title",
        "scope.title",
        "alias.other",
        "\"schema.name\".id",
    ];
    let joins = [
        Join::inner("Role", "User.id", "Role.user_id"),
        Join::cross("logs"),
        Join::condition(JoinKind::Inner, "extra", "a = b OR c"),
    ];
    let order = [OrderBy::desc("Name")];
    let select = SelectStatement::new(SelectStatementOptions {
        distinct: true,
        where_clause: Some(&where_clause),
        joins: &joins,
        group_by: &["Name"],
        having: Some(&having),
        order_by: &order,
        limit: Some(5),
        offset: Some(1),
        ..SelectStatementOptions::new("User", &fields)
    });
    let mapped = mappings().apply_select(&select, &cell).unwrap();
    assert_eq!(mapped.table, "people");
    assert_eq!(
        mapped.fields,
        [
            "full_name",
            "book.title",
            "book.title",
            "alias.other",
            "\"schema.name\".user_id"
        ]
    );
    assert!(mapped.distinct);
    assert_eq!(mapped.limit, Some(5));
    assert_eq!(mapped.offset, Some(1));
    assert_eq!(mapped.group_by, ["full_name"]);
    assert_eq!(mapped.order_by, [OrderBy::desc("full_name")]);
    assert_eq!(
        mapped.joins[0].on.as_deref(),
        Some("people.user_id = roles.user_id")
    );
    assert_eq!(mapped.joins[1].table, "logs");
    assert!(mapped.joins[1].on.is_none());
    assert_eq!(mapped.joins[2].on.as_deref(), Some("a = b OR c"));
    assert!(mapped.get_text().contains("full_name = $full_name"));
    let parameters = mapped.predicate().unwrap().parameters();
    assert_eq!(parameters[0].value, Value::Text("Name:BEA".into()));
    let names = mappings();
    assert_eq!(names.property("full_name"), "Name");
    assert_eq!(names.property("people.full_name"), "Name");
    assert_eq!(names.property("other"), "other");
    assert_eq!(names.property("people.missing"), "people.missing");
}

#[test]
fn maps_writes() {
    let names = mappings();
    let insert = names.apply_insert(&InsertStatement::new(
        "User",
        &[Parameter::new("Name", "Ada")],
    ));
    assert_eq!(
        insert.get_text(),
        "INSERT INTO people (full_name) VALUES ($full_name)"
    );
    assert_eq!(insert.fields()[0].value, Value::Text("Ada".into()));

    let where_clause = Where::new(Operator::equals("Name", "Bea"));
    let update = names
        .apply_update(
            &UpdateStatement::new(
                "User",
                &[Parameter::new("Name", "Ada")],
                Some(&where_clause),
            ),
            &cell,
        )
        .unwrap();
    assert_eq!(
        update.get_text(),
        "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where"
    );
    let parameters = update.parameters();
    assert_eq!(parameters[0].value, Value::Text("Ada".into()));
    assert_eq!(parameters[1].name, "full_name__where");
    assert_eq!(parameters[1].value, Value::Text("Name:BEA".into()));

    let delete = names
        .apply_delete(
            &DeleteStatement::new("User", Some(&where_clause)),
            &cell,
        )
        .unwrap();
    assert_eq!(
        delete.get_text(),
        "DELETE FROM people WHERE full_name = $full_name"
    );

    let inserted = names.apply_inserted(&SelectInsertedIdStatement::new("User", "id"));
    assert_eq!(inserted.table, "people");
    assert_eq!(inserted.column, "user_id");

    let bare = names
        .apply_update(
            &UpdateStatement::new("User", &[Parameter::new("Name", "Ada")], None),
            &cell,
        )
        .unwrap();
    assert_eq!(bare.get_text(), "UPDATE people SET full_name = $full_name");
}

#[test]
fn leaves_raw_and_other_statements() {
    let names = mappings();
    let raw_insert = InsertStatement::text("INSERT INTO t VALUES (1)");
    assert_eq!(
        names.apply_insert(&raw_insert).get_text(),
        "INSERT INTO t VALUES (1)"
    );
    let raw_update = UpdateStatement::text("x", None);
    assert_eq!(names.apply_update(&raw_update, &cell).unwrap().get_text(), "x");
    let raw_where = UpdateStatement::text("x", Some(Box::new(WhereText::new("id = 1"))));
    assert_eq!(
        names.apply_update(&raw_where, &cell).unwrap().get_text(),
        "x"
    );
    let raw_delete = DeleteStatement::raw("DELETE FROM t", None);
    assert_eq!(
        names.apply_delete(&raw_delete, &cell).unwrap().get_text(),
        "DELETE FROM t"
    );
    let text = WhereText::new("name = $name");
    let kept = names
        .apply_delete(&DeleteStatement::new("User", Some(&text)), &cell)
        .unwrap();
    assert_eq!(kept.get_text(), "DELETE FROM people WHERE name = $name");
    assert_eq!(
        ExecuteStoredProcedureStatement::text("CALL save_user()").get_text(),
        "CALL save_user()"
    );
}
