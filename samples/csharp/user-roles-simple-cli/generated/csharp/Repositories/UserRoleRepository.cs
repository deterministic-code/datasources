using Datasources;

public sealed class UserRoleRepository : CRUDRepository<Backend.Types.Datasource.UserRole>
{
    public UserRoleRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.UserRole> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.UserRole>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.UserRole> Options() => new()
    {
        Entity = "UserRole",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserRole"] = "user_roles" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["Created"] = "created",
            ["Updated"] = "updated",
            ["UserId"] = "user_id",
            ["RoleId"] = "bad_role_id",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Uuid", "Created", "Updated", "UserId", "RoleId"],
        CreateProperties = ["UserId", "RoleId"],
        UpdateProperties = ["Uuid", "UserId", "RoleId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, false),
            ["Updated"] = new("binary", false, true),
            ["UserId"] = new("integer", false, false),
            ["RoleId"] = new("integer", false, false),
        }),
    };
}
