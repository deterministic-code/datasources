namespace Datasources.Tests.Contact;

internal static class ExtraContact
{
    static readonly DateTime Stamp = DateTime.Parse("2024-01-01T00:00:00Z").ToUniversalTime();

    public static Contact New(ContactLookups ids) => new()
    {
        uuid = Guid.NewGuid(),
        created = Stamp,
        updated = Stamp,
        version = [],
        contact_source_id = ids.ContactSourceId,
        contact_type_id = ids.ContactTypeId,
        first_name = "Extra",
        last_name = "Contact",
        addresses =
        [
            new Address
            {
                uuid = Guid.NewGuid(),
                created = Stamp,
                updated = Stamp,
                version = [],
                address_type_id = ids.AddressTypeId,
                line1 = "9 Extra",
                city = "Oslo"
            }
        ],
        phones =
        [
            new Phone
            {
                uuid = Guid.NewGuid(),
                created = Stamp,
                updated = Stamp,
                version = [],
                phone_type_id = ids.PhoneTypeId,
                number = "555-extra"
            }
        ]
    };
}
