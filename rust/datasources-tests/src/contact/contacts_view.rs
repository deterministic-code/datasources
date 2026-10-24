use datasources::{TypeMap, TypeMapField, TypeMapGroup};

pub fn contacts_view() -> TypeMap {
    TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("Id")),
            ("first_name".into(), TypeMapField::column("FirstName")),
            ("last_name".into(), TypeMapField::column("LastName")),
            ("email".into(), TypeMapField::column("Email")),
            (
                "contact_source_name".into(),
                TypeMapField::column("Name"),
            ),
            (
                "contact_type_name".into(),
                TypeMapField::column("ContactTypeName"),
            ),
            (
                "addresses".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "AddressId",
                    vec![
                        ("id".into(), TypeMapField::column("AddressId")),
                        ("line1".into(), TypeMapField::column("Line1")),
                        ("city".into(), TypeMapField::column("City")),
                        (
                            "address_type_name".into(),
                            TypeMapField::column("AddressTypeName"),
                        ),
                    ],
                )),
            ),
            (
                "phones".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "PhoneId",
                    vec![
                        ("id".into(), TypeMapField::column("PhoneId")),
                        ("number".into(), TypeMapField::column("Number")),
                        (
                            "phone_type_name".into(),
                            TypeMapField::column("PhoneTypeName"),
                        ),
                    ],
                )),
            ),
        ],
    )
}
