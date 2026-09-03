using Datasources;

namespace Datasources.Tests.Contact;

internal sealed class ContactsService : DeepCRUDRepository<Contact>
{
    public ContactsService(
        ICRUDRepository<Contact> contacts,
        IReadonlyRepository<ContactType> types,
        IReadonlyRepository<ContactSource> sources,
        ICRUDRepository<Address> addresses,
        ICRUDRepository<Phone> phones)
        : base(contacts)
    {
        AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
        AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
        AddEnriched("addresses", "id", addresses, "contact_id", many: true);
        AddEnriched("phones", "id", phones, "contact_id", many: true);
    }
}
