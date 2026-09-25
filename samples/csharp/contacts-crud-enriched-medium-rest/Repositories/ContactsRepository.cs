using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactsRepository : CRUDRepository<ContactsView>
{
    public ContactSourceRepository Sources { get; }
    public ContactTypeRepository Types { get; }
    public AddressTypeRepository AddressTypes { get; }
    public PhoneTypeRepository PhoneTypes { get; }

    public ContactsRepository(IConnection connection) : base(connection, Options(connection.Provider))
    {
        Sources = new ContactSourceRepository(connection);
        Types = new ContactTypeRepository(connection);
        AddressTypes = new AddressTypeRepository(connection);
        PhoneTypes = new PhoneTypeRepository(connection);
        AddEnriched("contact_source", "contact_source_id", Sources, "id", "name");
        AddEnriched("contact_type", "contact_type_id", Types, "id", "name");
        AddEnriched("addresses", "id", new AddressRepository(connection), "contact_id", many: true,
            nest: node => node.Include(AddressTypes, "address_type", "address_type_id", "id", "name"));
        AddEnriched("phones", "id", new PhoneRepository(connection), "contact_id", many: true,
            nest: node => node.Include(PhoneTypes, "phone_type", "phone_type_id", "id", "name"));
        AddLinked("groups", "id", new ContactGroupRepository(connection), "id", new ContactGroupMemberRepository(connection), "contact_id", "contact_group_id");
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
