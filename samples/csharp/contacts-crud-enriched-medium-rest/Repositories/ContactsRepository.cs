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

    public ContactsRepository(IConnection connection) : base(connection, Options(connection.Provider))
    {
        Sources = new ContactSourceRepository(connection);
        Types = new ContactTypeRepository(connection);
        AddressTypes = new AddressTypeRepository(connection);
        PhoneTypes = new PhoneTypeRepository(connection);
        Addresses = new AddressRepository(connection);
        Phones = new PhoneRepository(connection);
        Groups = new ContactGroupRepository(connection);
        Members = new ContactGroupMemberRepository(connection);
        Addresses.AddEnriched("address_type", "address_type_id", AddressTypes, "id", "name");
        Phones.AddEnriched("phone_type", "phone_type_id", PhoneTypes, "id", "name");
        AddEnriched("contact_source", "contact_source_id", Sources, "id", "name");
        AddEnriched("contact_type", "contact_type_id", Types, "id", "name");
        AddEnriched("addresses", "id", Addresses, "contact_id", many: true);
        AddEnriched("phones", "id", Phones, "contact_id", many: true);
        AddLinked("groups", "id", Groups, "id", Members, "contact_id", "contact_group_id");
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

    static CRUDRepositoryOptions<ContactsView> Options(string provider) => new()
    {
        Entity = "contacts",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes"],
        Insert = provider == "mysql" ? MysqlCalls.Insert("add_contact") : null,
        Update = provider == "mysql" ? MysqlCalls.Update("update_contact") : null,
        Delete = provider == "mysql" ? MysqlCalls.Delete("delete_contact") : null
    };
}
