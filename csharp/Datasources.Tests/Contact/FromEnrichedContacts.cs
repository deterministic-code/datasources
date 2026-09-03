namespace Datasources.Tests.Contact;

internal static class FromEnrichedContacts
{
    public static IReadOnlyList<ContactView> Of(IEnumerable<Contact> contacts) =>
        [.. contacts.Select(contact => new ContactView(
            contact.id,
            contact.first_name,
            contact.last_name,
            contact.email,
            contact.contact_source_name,
            contact.contact_type_name,
            [.. contact.addresses.Select(address => new AddressView(address.id, address.line1, address.city))],
            [.. contact.phones.Select(phone => new PhoneView(phone.id, phone.number, phone.label))]))];
}
