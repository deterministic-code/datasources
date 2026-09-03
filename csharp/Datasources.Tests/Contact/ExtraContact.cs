using Datasources;
using Datasources.Builders;

namespace Datasources.Tests.Contact;

internal static class ExtraContact
{
    static readonly DateTime Stamp = DateTime.Parse("2024-01-01T00:00:00Z").ToUniversalTime();
    static readonly Guid Id = Guid.Parse("00000000-0000-4000-8000-000000000251");

    public static Contact New() => new()
    {
        id = 251,
        uuid = Id,
        created = Stamp,
        updated = Stamp,
        version = [],
        contact_source_id = 1,
        contact_type_id = 1,
        first_name = "Extra",
        last_name = "Contact",
        addresses =
        [
            new Address
            {
                id = 10001,
                uuid = Id,
                created = Stamp,
                updated = Stamp,
                version = [],
                line1 = "9 Extra",
                city = "Oslo"
            }
        ],
        phones =
        [
            new Phone
            {
                id = 10001,
                uuid = Id,
                created = Stamp,
                updated = Stamp,
                version = [],
                number = "555-extra"
            }
        ]
    };

    public static async Task<Contact> Add(ContactsService service)
    {
        var created = New();
        await service.Create(created);
        return await service.GetSingle(WhereBuilder.Equals("id", Value.From(created.id)))
            ?? throw new InvalidOperationException($"extra contact {created.id} missing");
    }
}
