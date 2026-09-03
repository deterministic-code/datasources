using Datasources;

namespace Datasources.Tests.Contact;

internal static class FromMappedContacts
{
    public static IReadOnlyList<ContactView> Of(IEnumerable<Dictionary<string, MappedValue>> rows) =>
        [.. rows.Select(row => new ContactView(
            IdOf(row, "id"),
            TextOf(row, "first_name"),
            TextOf(row, "last_name"),
            TextOf(row, "email") is { Length: > 0 } email ? email : null,
            TextOf(row, "contact_source_name"),
            TextOf(row, "contact_type_name"),
            [.. Many(row, "addresses").Select(child => new AddressView(
                IdOf(child, "id"), TextOf(child, "line1"), TextOf(child, "city")))],
            [.. Many(row, "phones").Select(child => new PhoneView(
                IdOf(child, "id"), TextOf(child, "number"),
                TextOf(child, "label") is { Length: > 0 } label ? label : null))]))];

    static IReadOnlyList<Dictionary<string, MappedValue>> Many(
        Dictionary<string, MappedValue> row, string key) =>
        row.TryGetValue(key, out var mapped) && mapped is MappedValue.Many(var items) ? items : [];

    static int IdOf(Dictionary<string, MappedValue> row, string key) =>
        row.TryGetValue(key, out var mapped) && mapped is MappedValue.Cell(var value)
            ? value switch
            {
                Value.Int(var number) => (int)number,
                _ => int.TryParse($"{value.ToObject()}", out var parsed) ? parsed : 0
            }
            : 0;

    static string TextOf(Dictionary<string, MappedValue> row, string key) =>
        row.TryGetValue(key, out var mapped) && mapped is MappedValue.Cell(var value)
            ? value switch
            {
                Value.Null => "",
                Value.Text(var text) => text,
                _ => value.ToObject()?.ToString() ?? ""
            }
            : "";
}
