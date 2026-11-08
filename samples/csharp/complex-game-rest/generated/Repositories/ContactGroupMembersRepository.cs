using Datasources;

public sealed class ContactGroupMembersRepository : CRUDRepository<Backend.Types.Datasource.ContactGroupMember>
{
    public ContactGroupMembersRepository(IConnection connection) : base(connection, Options(connection)) { }

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
                CreateProperties = ["ContactId", "ContactGroupId"],
                UpdateProperties = ["Uuid", "ContactId", "ContactGroupId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("biginteger", false, false),
                    ["ContactGroupId"] = new("biginteger", false, false),
                }),
            })
            .Add("sqlite", new()
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
                CreateProperties = ["Uuid", "Created", "Version", "ContactId", "ContactGroupId"],
                UpdateProperties = ["Uuid", "Created", "Version", "ContactId", "ContactGroupId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["ContactId"] = new("biginteger", false, false),
                    ["ContactGroupId"] = new("biginteger", false, false),
                }),
            })
            .For(connection);
}
