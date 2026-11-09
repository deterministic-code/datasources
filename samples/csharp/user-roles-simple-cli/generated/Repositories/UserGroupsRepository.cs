using Datasources;

public sealed class UserGroupsRepository : CRUDRepository<Backend.Types.Datasource.UserGroup>
{
    public UserGroupsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.UserGroup> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.UserGroup>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.UserGroup> Options() => new()
    {
        Entity = "UserGroup",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserGroup"] = "user_groups" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["UserId"] = "user_id",
            ["GroupId"] = "group_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "UserId", "GroupId"],
        CreateProperties = ["UserId", "GroupId"],
        UpdateProperties = ["Uuid", "UserId", "GroupId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, false),
            ["Updated"] = new("binary", false, true),
            ["UserId"] = new("integer", false, false),
            ["GroupId"] = new("integer", false, false),
        }),
    };
}
