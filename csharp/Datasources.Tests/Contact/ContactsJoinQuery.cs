using Datasources;
using Datasources.Builders;

namespace Datasources.Tests.Contact;

internal static class ContactsJoinQuery
{
    public static IQuery Create() =>
        StatementBuilder.CreateQuery(
            "contacts_base",
            [
                "contacts_base.id AS id",
                "contacts_base.first_name AS first_name",
                "contacts_base.last_name AS last_name",
                "contacts_base.email AS email",
                "contact_source.name AS contact_source_name",
                "contact_type.name AS contact_type_name",
                "address.id AS address_id",
                "address.line1 AS address_line1",
                "address.city AS address_city",
                "phone.id AS phone_id",
                "phone.number AS phone_number",
                "phone.label AS phone_label",
            ],
            null,
            [
                Join.Inner("contact_source", "contacts_base.contact_source_id", "contact_source.id"),
                Join.Inner("contact_type", "contacts_base.contact_type_id", "contact_type.id"),
                Join.Left("address", "contacts_base.id", "address.contact_id"),
                Join.Left("phone", "contacts_base.id", "phone.contact_id"),
            ]);
}
