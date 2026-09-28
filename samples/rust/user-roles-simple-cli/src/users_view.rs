use datasources::{TypeMap, TypeMapField, TypeMapGroup};

pub fn type_map() -> TypeMap {
    TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("username".into(), TypeMapField::column("username")),
            ("is_enabled".into(), TypeMapField::column("is_enabled")),
            ("created".into(), TypeMapField::column("created")),
            (
                "roles".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "role_id",
                    vec![
                        ("id".into(), TypeMapField::column("role_id")),
                        ("name".into(), TypeMapField::column("role_name")),
                        ("is_enabled".into(), TypeMapField::column("role_is_enabled")),
                    ],
                )),
            ),
            (
                "groups".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "group_id",
                    vec![
                        ("id".into(), TypeMapField::column("group_id")),
                        ("name".into(), TypeMapField::column("group_name")),
                        (
                            "is_enabled".into(),
                            TypeMapField::column("group_is_enabled"),
                        ),
                    ],
                )),
            ),
        ],
    )
}
