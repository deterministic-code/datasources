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
        addresses.AddEnriched("address_type_name", "address_type_id", addressTypes, "id", "name");
        phones.AddEnriched("phone_type_name", "phone_type_id", phoneTypes, "id", "name");
        contacts.AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
        contacts.AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
        contacts.AddEnriched("addresses", "id", addresses, "contact_id", many: true);
        contacts.AddEnriched("phones", "id", phones, "contact_id", many: true);
        return contacts;
    }
}
