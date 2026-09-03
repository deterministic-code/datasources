using Datasources;

namespace Datasources.Tests.Contact;

internal sealed class ContactsService : EnrichedReadonlyRepository<Contact>
{
    public ContactsService(
        IReadonlyRepository<Contact> contacts,
        IReadonlyRepository<ContactType> types,
        IReadonlyRepository<ContactSource> sources,
        IReadonlyRepository<Address> addresses,
        IReadonlyRepository<Phone> phones)
        : base(contacts)
    {
        AddEnriched("contact_type_name", "contact_type_id", types, "id", "name");
        AddEnriched("contact_source_name", "contact_source_id", sources, "id", "name");
        AddEnriched("addresses", "id", addresses, "contact_id", many: true);
        AddEnriched("phones", "id", phones, "contact_id", many: true);
    }
}
