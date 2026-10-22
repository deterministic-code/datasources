using Datasources;

public sealed class ContactGroupMemberRepository : CRUDRepository<Backend.Types.Datasource.ContactGroupMember>
{
    public ContactGroupMemberRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactGroupMember> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactGroupMember>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactGroupMember> Options() => new()
    {
        Entity = "ContactGroupMember",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactGroupMember"] = "contact_group_members" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["Version"] = "version",
            ["ContactId"] = "contact_id",
            ["ContactGroupId"] = "contact_group_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "ContactGroupId"],
    };
}
