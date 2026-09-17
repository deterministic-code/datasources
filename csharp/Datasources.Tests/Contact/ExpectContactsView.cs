namespace Datasources.Tests.Contact;

internal static class ExpectContactsView
{
    public static ContactView With(IEnumerable<ContactView> contacts, int addresses, int phones) =>
        contacts.First(contact => contact.addresses.Length == addresses && contact.phones.Length == phones);

    public static void AssertSeed(IReadOnlyList<ContactView> contacts, int count)
    {
        Assert.Equal(count, contacts.Count);
        _ = With(contacts, 2, 1);
        _ = With(contacts, 0, 1);
        _ = With(contacts, 2, 0);
        _ = With(contacts, 0, 0);
    }
}
