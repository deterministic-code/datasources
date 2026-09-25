using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroupMemberRepository : CRUDRepository<ContactGroupMember>
{
    public ContactGroupMemberRepository(IConnection connection) : base(connection, Options(connection.Provider)) { }

    static CRUDRepositoryOptions<ContactGroupMember> Options(string provider) => new()
    {
        Entity = "contact_group_member",
        EntityMapper = Names.Tables,
        ColumnMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "contact_group_id"],
        Insert = provider == "mysql" ? MysqlCalls.Insert("add_contact_group_member") : null,
        Delete = provider == "mysql" ? MysqlCalls.Delete("delete_contact_group_member") : null
    };
}
