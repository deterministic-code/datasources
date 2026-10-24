fn stmt_text(statement: &dyn crate::Statement) -> String {
    crate::DefaultStatementTextRenderer::instance().render(statement)
}

use std::sync::Arc;

use crate::mappers::{DictionaryMapper, Mapper};
use crate::*;

fn mappings() -> StatementMappers {
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
    StatementMappers::new(entities, properties)
}

fn mapper(name: &str, value: &Value) -> Result<Value> {
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
    let mapped = mappings().apply_select(&select, &mapper).unwrap();
    assert_eq!(mapped.table, "people");
    assert_eq!(
        mapped.fields,
        [
            "full_name",
            "book.title",
            "book.title AS title",
            "alias.other AS other",
            "\"schema.name\".user_id AS id"
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
    assert!(stmt_text(&mapped).contains("full_name = $full_name"));
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
        stmt_text(&insert),
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
            &mapper,
        )
        .unwrap();
    assert_eq!(
        stmt_text(&update),
        "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where"
    );
    let parameters = update.parameters();
    assert_eq!(parameters[0].value, Value::Text("Ada".into()));
    assert_eq!(parameters[1].name, "full_name__where");
    assert_eq!(parameters[1].value, Value::Text("Name:BEA".into()));

    let delete = names
        .apply_delete(
            &DeleteStatement::new("User", Some(&where_clause)),
            &mapper,
        )
        .unwrap();
    assert_eq!(
        stmt_text(&delete),
        "DELETE FROM people WHERE full_name = $full_name"
    );

    let inserted = names.apply_inserted(&SelectInsertedIdStatement::new("User", "id"));
    assert_eq!(inserted.table, "people");
    assert_eq!(inserted.column, "user_id");

    let bare = names
        .apply_update(
            &UpdateStatement::new("User", &[Parameter::new("Name", "Ada")], None),
            &mapper,
        )
        .unwrap();
    assert_eq!(stmt_text(&bare), "UPDATE people SET full_name = $full_name");
}

#[test]
fn leaves_raw_and_other_statements() {
    let names = mappings();
    let call = ExecuteStoredProcedureStatement::new("save_user", &[]);
    assert_eq!(stmt_text(&call), "CALL save_user()");
    assert!(ReadInput::as_where(&WhereText::new("id = 1")).is_some());
    let text = WhereText::new("name = $name");
    let kept = names
        .apply_delete(&DeleteStatement::new("User", Some(&text)), &mapper)
        .unwrap();
    assert_eq!(stmt_text(&kept), "DELETE FROM people WHERE name = $name");
}

#[test]
fn maps_select_field_alias() {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contacts_base".to_string()),
        ("Address".to_string(), "address".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([
            ("Id".to_string(), "id".to_string()),
            ("FirstName".to_string(), "first_name".to_string()),
            ("ContactId".to_string(), "contact_id".to_string()),
        ]));
    let mappings = StatementMappers::new(entities, properties);
    let fields = [
        "Contact.Id",
        "Contact.FirstName",
        "Address.Id",
    ];
    let joins = [Join::left("Address", "Contact.Id", "Address.ContactId")];
    let select = SelectStatement::new(SelectStatementOptions {
        joins: &joins,
        ..SelectStatementOptions::new("Contact", &fields)
    });
    let mapped = mappings.apply_select(&select, &mapper).unwrap();
    assert_eq!(
        stmt_text(&mapped),
        "SELECT contacts_base.id AS Id, contacts_base.first_name AS FirstName, address.id AS AddressId FROM contacts_base LEFT JOIN address ON contacts_base.id = address.contact_id"
    );
}

#[test]
fn maps_contacts_select() {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contacts_base".to_string()),
        ("ContactSource".to_string(), "contact_source".to_string()),
        ("ContactType".to_string(), "contact_type".to_string()),
        ("Address".to_string(), "address".to_string()),
        ("AddressType".to_string(), "address_type".to_string()),
        ("Phone".to_string(), "phone".to_string()),
        ("PhoneType".to_string(), "phone_type".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([
            ("Id".to_string(), "id".to_string()),
            ("FirstName".to_string(), "first_name".to_string()),
            ("LastName".to_string(), "last_name".to_string()),
            ("Email".to_string(), "email".to_string()),
            ("Name".to_string(), "name".to_string()),
            ("Line1".to_string(), "line1".to_string()),
            ("City".to_string(), "city".to_string()),
            ("Number".to_string(), "number".to_string()),
            ("ContactId".to_string(), "contact_id".to_string()),
            ("ContactSourceId".to_string(), "contact_source_id".to_string()),
            ("ContactTypeId".to_string(), "contact_type_id".to_string()),
            ("AddressTypeId".to_string(), "address_type_id".to_string()),
            ("PhoneTypeId".to_string(), "phone_type_id".to_string()),
        ]));
    let mappings = StatementMappers::new(entities, properties);
    let fields = [
        "Contact.FirstName",
        "Contact.LastName",
        "Contact.Email",
        "ContactSource.Name",
        "Address.Line1",
        "Address.City",
        "Phone.Number",
    ];
    let joins = [
        Join::inner(
            "ContactSource",
            "Contact.ContactSourceId",
            "ContactSource.Id",
        ),
        Join::inner("ContactType", "Contact.ContactTypeId", "ContactType.Id"),
        Join::left("Address", "Contact.Id", "Address.ContactId"),
        Join::left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
        Join::left("Phone", "Contact.Id", "Phone.ContactId"),
        Join::left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
    ];
    let where_clause = Where::new(Operator::and(
        Operator::equals("Contact.Email", "ada@ex.com"),
        Operator::greater_than("Address.City", "A"),
    ));
    let having = Where::new(Operator::equals("ContactType.Name", "person"));
    let order = [
        OrderBy::asc("Contact.LastName"),
        OrderBy::desc("Contact.FirstName"),
    ];
    let select = SelectStatement::new(SelectStatementOptions {
        distinct: true,
        where_clause: Some(&where_clause),
        joins: &joins,
        group_by: &["Contact.LastName"],
        having: Some(&having),
        order_by: &order,
        limit: Some(20),
        offset: Some(1),
        ..SelectStatementOptions::new("Contact", &fields)
    });
    let mapped = mappings.apply_select(&select, &mapper).unwrap();
    assert_eq!(
        stmt_text(&mapped),
        "SELECT DISTINCT contacts_base.first_name AS FirstName, contacts_base.last_name AS LastName, contacts_base.email AS Email, contact_source.name AS Name, address.line1 AS Line1, address.city AS City, phone.number AS Number FROM contacts_base INNER JOIN contact_source ON contacts_base.contact_source_id = contact_source.id INNER JOIN contact_type ON contacts_base.contact_type_id = contact_type.id LEFT JOIN address ON contacts_base.id = address.contact_id LEFT JOIN address_type ON address.address_type_id = address_type.id LEFT JOIN phone ON contacts_base.id = phone.contact_id LEFT JOIN phone_type ON phone.phone_type_id = phone_type.id WHERE contacts_base.email = $contacts_base.email AND address.city > $address.city GROUP BY contacts_base.last_name HAVING contact_type.name = $contact_type.name ORDER BY contacts_base.last_name ASC, contacts_base.first_name DESC LIMIT 20 OFFSET 1"
    );
    let parameters = mapped.predicate().unwrap().parameters();
    assert_eq!(
        parameters[0].value,
        Value::Text("Contact.Email:ADA@EX.COM".into())
    );
    assert_eq!(parameters[1].value, Value::Text("Address.City:A".into()));
}

#[test]
fn rejects_select_field_alias() {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contacts_base".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([("Id".to_string(), "id".to_string())]));
    let mappings = StatementMappers::new(entities, properties);
    let fields = ["Contact.Id AS Id"];
    let select = SelectStatement::new(SelectStatementOptions::new("Contact", &fields));
    let error = match mappings.apply_select(&select, &mapper) {
        Err(error) => error,
        Ok(_) => panic!("expected a path field error"),
    };
    assert_eq!(
        error.to_string(),
        "Select field \"Contact.Id AS Id\" must be a path"
    );
}

#[test]
fn maps_aggregate_alias_from_lookup() {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contacts_base".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([("Id".to_string(), "id".to_string())]));
    let mappings = StatementMappers::new(entities, properties);
    let count = Aggregate::count("Contact.Id");
    let fields = [count.as_str()];
    let select = SelectStatement::new(SelectStatementOptions {
        aliases: &[Some("n")],
        ..SelectStatementOptions::new("Contact", &fields)
    });
    let mapped = mappings.apply_select(&select, &mapper).unwrap();
    assert_eq!(mapped.fields, ["COUNT(contacts_base.id) AS n"]);
}

#[test]
fn maps_nested_path_alias() {
    let entities: Arc<dyn Mapper<String, String> + Send + Sync> = Arc::new(DictionaryMapper::new([
        ("Contact".to_string(), "contacts_base".to_string()),
        ("Address".to_string(), "address".to_string()),
        ("AddressType".to_string(), "address_type".to_string()),
        ("ContactSource".to_string(), "contact_source".to_string()),
    ]));
    let properties: Arc<dyn Mapper<String, String> + Send + Sync> =
        Arc::new(DictionaryMapper::new([
            ("Id".to_string(), "id".to_string()),
            ("City".to_string(), "city".to_string()),
            ("Name".to_string(), "name".to_string()),
        ]));
    let mappings = StatementMappers::new(entities, properties);
    let fields = [
        "Contact.Id",
        "Contact.Address.Id",
        "Contact.Address.City",
        "Contact.ContactSource.Name",
        "Contact.Address.AddressType.Name",
    ];
    let select = SelectStatement::new(SelectStatementOptions::new("Contact", &fields));
    let mapped = mappings.apply_select(&select, &mapper).unwrap();
    assert_eq!(
        mapped.fields,
        [
            "contacts_base.id AS Id",
            "address.id AS AddressId",
            "address.city AS City",
            "contact_source.name AS Name",
            "address_type.name AS AddressTypeName"
        ]
    );
}
