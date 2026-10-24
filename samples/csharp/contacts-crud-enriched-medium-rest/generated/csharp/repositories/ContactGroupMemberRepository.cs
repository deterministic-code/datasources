using Datasources;

public sealed class ContactGroupMemberRepository : CRUDRepository<Backend.Types.Datasource.ContactGroupMember>
{
    public ContactGroupMemberRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactGroupMember> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactGroupMember>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactGroupMember> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.ContactGroupMember>()
            .Add(new()
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
                CreateProperties = ["Uuid", "Version", "ContactId", "ContactGroupId"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "ContactGroupId"],
            })
            .Add("mysql", new()
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
                CreateProperties = ["ContactId", "ContactGroupId"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "ContactGroupId"],
            })
            .Add("postgres", new()
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
                CreateProperties = ["ContactId", "ContactGroupId"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "ContactGroupId"],
            })
            .For(connection);
}
