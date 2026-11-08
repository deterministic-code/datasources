using Datasources;

public sealed class UserGroupsRepository : CRUDRepository<Backend.Types.Datasource.UserGroup>
{
    public UserGroupsRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.UserGroup> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.UserGroup>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.UserGroup> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.UserGroup>()
            .Add(new()
            {
                Entity = "UserGroup",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserGroup"] = "user_groups" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["UserId"] = "user_id",
                    ["GroupId"] = "group_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "UserId", "GroupId"],
                CreateProperties = ["UserId", "GroupId"],
                UpdateProperties = ["Uuid", "UserId", "GroupId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["UserId"] = new("biginteger", false, false),
                    ["GroupId"] = new("biginteger", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "UserGroup",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserGroup"] = "user_groups" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["UserId"] = "user_id",
                    ["GroupId"] = "group_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "UserId", "GroupId"],
                CreateProperties = ["Uuid", "Created", "Version", "UserId", "GroupId"],
                UpdateProperties = ["Uuid", "Created", "Version", "UserId", "GroupId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["UserId"] = new("biginteger", false, false),
                    ["GroupId"] = new("biginteger", false, false),
                }),
            })
            .For(connection);
}
