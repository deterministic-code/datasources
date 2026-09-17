use datasources::{Join, StatementBuilder, TextQuery};

pub fn contacts_join_query() -> TextQuery {
    StatementBuilder::create_query(
        "contacts_base",
        &[
            "contacts_base.id AS id",
            "contacts_base.first_name AS first_name",
            "contacts_base.last_name AS last_name",
            "contacts_base.email AS email",
            "contact_source.name AS contact_source_name",
            "contact_type.name AS contact_type_name",
            "address.id AS address_id",
            "address.line1 AS address_line1",
            "address.city AS address_city",
            "address_type.name AS address_type_name",
            "phone.id AS phone_id",
            "phone.number AS phone_number",
            "phone_type.name AS phone_type_name",
        ],
        None,
        &[
            Join::inner(
                "contact_source",
                "contacts_base.contact_source_id",
                "contact_source.id",
            ),
            Join::inner(
                "contact_type",
                "contacts_base.contact_type_id",
                "contact_type.id",
            ),
            Join::left("address", "contacts_base.id", "address.contact_id"),
            Join::left("address_type", "address.address_type_id", "address_type.id"),
            Join::left("phone", "contacts_base.id", "phone.contact_id"),
            Join::left("phone_type", "phone.phone_type_id", "phone_type.id"),
        ],
    )
}
