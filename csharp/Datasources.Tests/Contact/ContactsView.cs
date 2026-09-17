using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsView
{
    public static TypeMap TypeMap { get; } = new()
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "id",
            ["first_name"] = "first_name",
            ["last_name"] = "last_name",
            ["email"] = "email",
            ["contact_source_name"] = "contact_source_name",
            ["contact_type_name"] = "contact_type_name",
            ["addresses"] = new TypeMapGroup
            {
                Key = "address_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "address_id",
                    ["line1"] = "address_line1",
                    ["city"] = "address_city",
                    ["address_type_name"] = "address_type_name",
                },
            },
            ["phones"] = new TypeMapGroup
            {
                Key = "phone_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "phone_id",
                    ["number"] = "phone_number",
                    ["phone_type_name"] = "phone_type_name",
                },
            },
        },
    };
}
