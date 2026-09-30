namespace Datasources.Tests.Contact;

internal static class ExtraContact
{
    static readonly DateTime Stamp = DateTime.Parse("2024-01-01T00:00:00Z").ToUniversalTime();

    public static Contact New(ContactLookups ids) => new()
    {
        Uuid = Guid.NewGuid(),
        Created = Stamp,
        Updated = Stamp,
        Version = [],
        ContactSource = new() { Id = ids.ContactSourceId },
        ContactType = new() { Id = ids.ContactTypeId },
        FirstName = "Extra",
        LastName = "Contact",
        Addresses =
        [
            new Address
            {
                Uuid = Guid.NewGuid(),
                Created = Stamp,
                Updated = Stamp,
                Version = [],
                AddressType = new() { Id = ids.AddressTypeId },
                Line1 = "9 Extra",
                City = "Oslo"
            }
        ],
        Phones =
        [
            new Phone
            {
                Uuid = Guid.NewGuid(),
                Created = Stamp,
                Updated = Stamp,
                Version = [],
                PhoneType = new() { Id = ids.PhoneTypeId },
                Number = "555-extra"
            }
        ]
    };
}
