use crate::*;

fn contact_graph() -> NestedSelectType {
    let source = NestedSelectType::new("ContactSource", "Id", vec!["Id".into(), "Name".into()], vec![]);
    let type_def = NestedSelectType::new("ContactType", "Id", vec!["Id".into(), "Name".into()], vec![]);
    let address_type = NestedSelectType::new("AddressType", "Id", vec!["Id".into(), "Name".into()], vec![]);
    let phone_type = NestedSelectType::new("PhoneType", "Id", vec!["Id".into(), "Name".into()], vec![]);
    let address = NestedSelectType::new(
        "Address",
        "Id",
        vec!["Id".into(), "Line1".into(), "City".into()],
        vec![NestedSelectRelation::new(
            "AddressType",
            NestedSelectRelationKind::Object,
            "AddressTypeId",
            "Id",
            address_type,
        )],
    );
    let phone = NestedSelectType::new(
        "Phone",
        "Id",
        vec!["Id".into(), "Number".into()],
        vec![NestedSelectRelation::new(
            "PhoneType",
            NestedSelectRelationKind::Object,
            "PhoneTypeId",
            "Id",
            phone_type,
        )],
    );
    NestedSelectType::new(
        "Contact",
        "Id",
        vec!["Id".into(), "FirstName".into(), "LastName".into(), "Email".into()],
        vec![
            NestedSelectRelation::new(
                "ContactSource",
                NestedSelectRelationKind::Object,
                "ContactSourceId",
                "Id",
                source,
            ),
            NestedSelectRelation::new(
                "ContactType",
                NestedSelectRelationKind::Object,
                "ContactTypeId",
                "Id",
                type_def,
            ),
            NestedSelectRelation::new("Addresses", NestedSelectRelationKind::List, "Id", "ContactId", address),
            NestedSelectRelation::new("Phones", NestedSelectRelationKind::List, "Id", "ContactId", phone),
        ],
    )
}

#[test]
fn builds_contact_joins_and_map() {
    let nested = NestedSelectBuilder::build(&contact_graph(), NestedSelectBuilderOptions::default());
    assert_eq!(nested.select.table, "Contact");
    assert_eq!(nested.select.fields.len(), 17);
    assert_eq!(nested.select.joins.len(), 6);
    assert_eq!(nested.map.key, "Id");
    assert!(nested.map.fields.iter().any(|(name, field)| {
        name == "Addresses"
            && matches!(field, TypeMapField::Group(group) if group.key == "AddressId")
    }));
}

#[test]
fn stop_at_drops_phones() {
    let nested = NestedSelectBuilder::build(
        &contact_graph(),
        NestedSelectBuilderOptions {
            stop_at: vec!["Phones".into()],
            ..NestedSelectBuilderOptions::default()
        },
    );
    assert!(nested.select.fields.iter().all(|field| !field.contains("Phone")));
    assert!(nested.select.joins.iter().all(|join| !join.table.contains("Phone")));
    assert!(nested.map.fields.iter().all(|(name, _)| name != "Phones"));
}

#[test]
fn procedure_adds_where_argument() {
    let procedure = CreateNestedReadStoredProcedureStatement::new(
        "ReadContactsByEmail",
        NestedSelectBuilder::build(
            &contact_graph(),
            NestedSelectBuilderOptions {
                stop_at: vec!["Phones".into()],
                ..NestedSelectBuilderOptions::default()
            },
        ),
        vec![NestedWhereArgument::new(
            "Contact.Email",
            RoutineParameter {
                name: "email".into(),
                r#type: "varchar".into(),
                field: None,
            },
        )],
    );
    let sql = DefaultStatementTextRenderer::instance().render(&procedure.select());
    assert!(sql.contains("Contact.Email"));
    assert!(!sql.contains("Phone"));
}

#[test]
fn procedure_ands_onto_existing_where() {
    let where_clause = Where::new(Operator::equals("Contact.Id", 1));
    let nested = NestedSelectBuilder::build(
        &contact_graph(),
        NestedSelectBuilderOptions {
            stop_at: vec!["Phones".into()],
            where_clause: Some(&where_clause),
        },
    );
    let procedure = CreateNestedReadStoredProcedureStatement::new(
        "ReadContactsByEmail",
        nested,
        vec![NestedWhereArgument::new(
            "Contact.Email",
            RoutineParameter {
                name: "email".into(),
                r#type: "varchar".into(),
                field: None,
            },
        )],
    );
    let sql = DefaultStatementTextRenderer::instance().render(&procedure.select());
    assert!(sql.contains(" AND "));
}
