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
        addresses.AddEnriched(new("address_type_name", "address_type_id", "id", "name"), addressTypes);
        phones.AddEnriched(new("phone_type_name", "phone_type_id", "id", "name"), phoneTypes);
        contacts.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), types);
        contacts.AddEnriched(new("contact_source_name", "contact_source_id", "id", "name"), sources);
        contacts.AddEnriched(new("addresses", "id", "contact_id", Many: true), addresses);
        contacts.AddEnriched(new("phones", "id", "contact_id", Many: true), phones);
        return contacts;
    }
}
