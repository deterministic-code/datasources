using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsView
{
    public static TypeMap TypeMap { get; } = new()
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "Id",
            ["first_name"] = "FirstName",
            ["last_name"] = "LastName",
            ["email"] = "Email",
            ["contact_source_name"] = "Name",
            ["contact_type_name"] = "ContactTypeName",
            ["addresses"] = new TypeMapGroup
            {
                Key = "AddressId",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "AddressId",
                    ["line1"] = "Line1",
                    ["city"] = "City",
                    ["address_type_name"] = "AddressTypeName",
                },
            },
            ["phones"] = new TypeMapGroup
            {
                Key = "PhoneId",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "PhoneId",
                    ["number"] = "Number",
                    ["phone_type_name"] = "PhoneTypeName",
                },
            },
        },
    };
}
