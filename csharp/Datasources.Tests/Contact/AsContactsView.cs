using Datasources;

namespace Datasources.Tests.Contact;

internal static class AsContactsView
{
    public static IReadOnlyList<ContactView> FromContacts(IEnumerable<Contact> contacts) =>
        contacts
            .Select(contact => new ContactView(
                contact.Id,
                contact.FirstName,
                contact.LastName,
                contact.Email,
                contact.ContactSource.Name,
                contact.ContactType.Name,
                [.. contact.Addresses
                    .Select(address => new AddressView(
                        address.Id, address.Line1, address.City, address.AddressType.Name))
                    .OrderBy(address => address.id)],
                [.. contact.Phones
                    .Select(phone => new PhoneView(phone.Id, phone.Number, phone.PhoneType.Name))
                    .OrderBy(phone => phone.id)]))
            .OrderBy(contact => contact.id)
            .ToList();

    public static IReadOnlyList<ContactView> FromMapped(
        IEnumerable<Dictionary<string, MappedValue>> rows) =>
        rows
            .Select(row => new ContactView(
                Id(row, "id"),
                Text(row, "first_name"),
                Text(row, "last_name"),
                NullText(row, "email"),
                Text(row, "contact_source_name"),
                Text(row, "contact_type_name"),
                [.. Many(row, "addresses")
                    .Select(address => new AddressView(
                        Id(address, "id"),
                        Text(address, "line1"),
                        Text(address, "city"),
                        Text(address, "address_type_name")))
                    .OrderBy(address => address.id)],
                [.. Many(row, "phones")
                    .Select(phone => new PhoneView(
                        Id(phone, "id"),
                        Text(phone, "number"),
                        Text(phone, "phone_type_name")))
                    .OrderBy(phone => phone.id)]))
            .OrderBy(contact => contact.id)
            .ToList();

    static List<Dictionary<string, MappedValue>> Many(
        Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Many(var items) ? items : [];

    static string Text(Dictionary<string, MappedValue> row, string name) =>
        Cell(row, name) switch
        {
            Value.Text(var text) => text,
            Value.Null or null => "",
            var value => value.ToObject()?.ToString() ?? ""
        };

    static string? NullText(Dictionary<string, MappedValue> row, string name) =>
        Cell(row, name) is Value.Text(var text) ? text : null;

    static int Id(Dictionary<string, MappedValue> row, string name) =>
        Cell(row, name) switch
        {
            Value.Int(var number) => (int)number,
            var value => int.TryParse(value?.ToObject()?.ToString(), out var parsed) ? parsed : 0
        };

    static Value? Cell(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var value) ? value : null;
}
