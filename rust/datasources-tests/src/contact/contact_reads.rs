use datasources::{
    CreateNestedReadStoredProcedureStatement, NestedSelectBuilder, NestedSelectBuilderOptions,
    NestedSelectRelation, NestedSelectRelationKind, NestedSelectStatement, NestedSelectType,
    NestedWhereArgument, RoutineParameter,
};

pub fn contact_graph() -> NestedSelectType {
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
        vec![
            "Id".into(),
            "FirstName".into(),
            "LastName".into(),
            "Email".into(),
        ],
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
            NestedSelectRelation::new(
                "Addresses",
                NestedSelectRelationKind::List,
                "Id",
                "ContactId",
                address,
            ),
            NestedSelectRelation::new(
                "Phones",
                NestedSelectRelationKind::List,
                "Id",
                "ContactId",
                phone,
            ),
        ],
    )
}

pub fn contact_reads_all() -> NestedSelectStatement {
    NestedSelectBuilder::build(&contact_graph(), NestedSelectBuilderOptions::default())
}

pub fn contact_reads_without_phones(email_type: &str) -> CreateNestedReadStoredProcedureStatement {
    CreateNestedReadStoredProcedureStatement::new(
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
                r#type: email_type.into(),
                field: None,
            },
        )],
    )
}
