using Datasources;

namespace Datasources.Tests.Contact;

internal sealed class ContactsService : DeepCRUDRepository<Contact>
{
    public ContactsService(
        ICRUDRepository<Contact> contacts,
        IReadonlyRepository<ContactType> types,
        IReadonlyRepository<ContactSource> sources,
        IReadonlyRepository<AddressType> addressTypes,
        IReadonlyRepository<PhoneType> phoneTypes,
        ICRUDRepository<Address> addresses,
        ICRUDRepository<Phone> phones)
        : base(contacts)
    {
        AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
        AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
        AddEnriched("addresses", "id", addresses, "contact_id", many: true,
            nest: node => node.Include(addressTypes, "address_type_name", "address_type_id", "id", "name"));
        AddEnriched("phones", "id", phones, "contact_id", many: true,
            nest: node => node.Include(phoneTypes, "phone_type_name", "phone_type_id", "id", "name"));
    }
}
