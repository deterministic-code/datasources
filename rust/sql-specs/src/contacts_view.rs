use datasources::{TypeMap, TypeMapField, TypeMapGroup};

pub fn contacts_view() -> TypeMap {
    TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("first_name".into(), TypeMapField::column("first_name")),
            ("last_name".into(), TypeMapField::column("last_name")),
            ("email".into(), TypeMapField::column("email")),
            (
                "contact_source_name".into(),
                TypeMapField::column("contact_source_name"),
            ),
            (
                "contact_type_name".into(),
                TypeMapField::column("contact_type_name"),
            ),
            (
                "addresses".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "address_id",
                    vec![
                        ("id".into(), TypeMapField::column("address_id")),
                        ("line1".into(), TypeMapField::column("address_line1")),
                        ("city".into(), TypeMapField::column("address_city")),
                        (
                            "address_type_name".into(),
                            TypeMapField::column("address_type_name"),
                        ),
                    ],
                )),
            ),
            (
                "phones".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "phone_id",
                    vec![
                        ("id".into(), TypeMapField::column("phone_id")),
                        ("number".into(), TypeMapField::column("phone_number")),
                        (
                            "phone_type_name".into(),
                            TypeMapField::column("phone_type_name"),
                        ),
                    ],
                )),
            ),
        ],
    )
}
