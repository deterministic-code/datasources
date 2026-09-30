using Datasources;

namespace Datasources.Tests.Contact;

internal static class ContactsService
{
    public static CRUDRepository<Contact> Open(
        CRUDRepository<Contact> contacts,
        IReadonlyRepository<ContactType> types,
        IReadonlyRepository<ContactSource> sources,
        IReadonlyRepository<AddressType> addressTypes,
        IReadonlyRepository<PhoneType> phoneTypes,
        CRUDRepository<Address> addresses,
        CRUDRepository<Phone> phones)
    {
        addresses.AddEnriched(new("AddressType", "AddressType", "Id"), addressTypes);
        phones.AddEnriched(new("PhoneType", "PhoneType", "Id"), phoneTypes);
        contacts.AddEnriched(new("ContactType", "ContactType", "Id"), types);
        contacts.AddEnriched(new("ContactSource", "ContactSource", "Id"), sources);
        contacts.AddEnriched(new("Addresses", "Id", "Contact", Many: true), addresses);
        contacts.AddEnriched(new("Phones", "Id", "Contact", Many: true), phones);
        return contacts;
    }
}
