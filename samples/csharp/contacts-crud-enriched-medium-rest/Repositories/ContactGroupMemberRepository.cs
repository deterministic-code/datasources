using Datasources;

namespace ContactsCrudEnrichedMediumRest;

public sealed class ContactGroupMemberRepository : CRUDRepository<ContactGroupMember>
{
    public ContactGroupMemberRepository(IConnection connection) : base(connection, Options()) { }

    static CRUDRepositoryOptions<ContactGroupMember> Options() => new()
    {
        Entity = "contact_group_member",
        EntityMapper = Names.Tables,
        PropertyMappings = Names.Columns,
        ReadProperties = ["id", "uuid", "created", "updated", "version", "contact_id", "contact_group_id"],
    };
}
