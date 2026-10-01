using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactsRepository : CRUDRepository<ContactsView>
{
    public ContactSourceRepository Sources { get; }
    public ContactTypeRepository Types { get; }
    public AddressTypeRepository AddressTypes { get; }
    public PhoneTypeRepository PhoneTypes { get; }
    public AddressRepository Addresses { get; }
    public PhoneRepository Phones { get; }
    public ContactGroupRepository Groups { get; }
    public ContactGroupMemberRepository Members { get; }

    public ContactsRepository(IConnection connection) : base(connection, Options())
    {
        Sources = new ContactSourceRepository(connection);
        Types = new ContactTypeRepository(connection);
        AddressTypes = new AddressTypeRepository(connection);
        PhoneTypes = new PhoneTypeRepository(connection);
        Addresses = new AddressRepository(connection);
        Phones = new PhoneRepository(connection);
        Groups = new ContactGroupRepository(connection);
        Members = new ContactGroupMemberRepository(connection);
        Addresses.AddEnriched(new("address_type", "address_type_id", "id", "name"), AddressTypes);
        Phones.AddEnriched(new("phone_type", "phone_type_id", "id", "name"), PhoneTypes);
        AddEnriched(new("contact_source", "contact_source_id", "id", "name"), Sources);
        AddEnriched(new("contact_type", "contact_type_id", "id", "name"), Types);
        AddEnriched(new("addresses", "id", "contact_id", Type: EnrichmentPropertyType.List), Addresses);
        AddEnriched(new("phones", "id", "contact_id", Type: EnrichmentPropertyType.List), Phones);
        AddLinked(new("groups", "id", "id", Association: new("contact_id", "contact_group_id")), Groups, Members);
    }

    public new ContactsRepository AddMiddleware(ICommandMiddleware middleware)
    {
        base.AddMiddleware(middleware);
        Sources.AddMiddleware(middleware);
        Types.AddMiddleware(middleware);
        AddressTypes.AddMiddleware(middleware);
        PhoneTypes.AddMiddleware(middleware);
        Addresses.AddMiddleware(middleware);
        Phones.AddMiddleware(middleware);
        Groups.AddMiddleware(middleware);
        Members.AddMiddleware(middleware);
        return this;
    }

    static CRUDRepositoryOptions<ContactsView> Options() => new()
    {
        Entity = "contacts",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes"],
    };
}
