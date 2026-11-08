using Datasources;

public sealed class UserRolesRepository : CRUDRepository<Backend.Types.Datasource.UserRole>
{
    public UserRolesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.UserRole> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.UserRole>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.UserRole> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.UserRole>()
            .Add(new()
            {
                Entity = "UserRole",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserRole"] = "user_roles" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["UserId"] = "user_id",
                    ["RoleId"] = "role_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "UserId", "RoleId"],
                CreateProperties = ["UserId", "RoleId"],
                UpdateProperties = ["Uuid", "UserId", "RoleId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["UserId"] = new("biginteger", false, false),
                    ["RoleId"] = new("biginteger", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "UserRole",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["UserRole"] = "user_roles" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["UserId"] = "user_id",
                    ["RoleId"] = "role_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "UserId", "RoleId"],
                CreateProperties = ["Uuid", "Created", "Version", "UserId", "RoleId"],
                UpdateProperties = ["Uuid", "Created", "Version", "UserId", "RoleId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["UserId"] = new("biginteger", false, false),
                    ["RoleId"] = new("biginteger", false, false),
                }),
            })
            .For(connection);
}
