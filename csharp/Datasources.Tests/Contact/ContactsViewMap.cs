using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsViewMap
{
    public static TypeMap Spec { get; } = new()
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
                },
            },
            ["phones"] = new TypeMapGroup
            {
                Key = "phone_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "phone_id",
                    ["number"] = "phone_number",
                    ["label"] = "phone_label",
                },
            },
        },
    };
}
