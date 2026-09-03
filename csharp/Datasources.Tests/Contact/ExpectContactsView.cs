namespace Datasources.Tests.Contact;

internal static class ExpectContactsView
{
    public static void Of(IReadOnlyList<ContactView> contacts, int count)
    {
        ContactView? At(int id) => contacts.FirstOrDefault(contact => contact.Id == id);
        Assert.Equal(count, contacts.Count);
        Assert.Equal(2, At(10)?.Addresses.Length);
        Assert.Equal(1, At(10)?.Phones.Length);
        Assert.Empty(At(4)?.Addresses ?? []);
        Assert.Equal(1, At(4)?.Phones.Length);
        Assert.Equal(2, At(6)?.Addresses.Length);
        Assert.Empty(At(6)?.Phones ?? []);
        Assert.Empty(At(12)?.Addresses ?? []);
        Assert.Empty(At(12)?.Phones ?? []);
    }
}
