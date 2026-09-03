namespace Datasources.Tests.Contact;

internal static class AsContactsView
{
    public static IReadOnlyList<ContactView> Of(IEnumerable<ContactView> contacts) =>
        [.. contacts.OrderBy(contact => contact.Id).Select(contact => contact with
        {
            Addresses = [.. contact.Addresses.OrderBy(address => address.Id)],
            Phones = [.. contact.Phones.OrderBy(phone => phone.Id)],
        })];
}
