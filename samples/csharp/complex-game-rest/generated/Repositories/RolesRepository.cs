using Datasources;

public sealed class RolesRepository : CRUDRepository<Backend.Types.Datasource.Role>
{
    public RolesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Role> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Role>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.Role> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.Role>()
            .Add(new()
            {
                Entity = "Role",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Role"] = "roles" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description", "IsEnabled"],
                CreateProperties = ["Name", "Description", "IsEnabled"],
                UpdateProperties = ["Uuid", "Name", "Description", "IsEnabled"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", false, true),
                    ["IsEnabled"] = new("boolean", false, true),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Role",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Role"] = "roles" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description", "IsEnabled"],
                CreateProperties = ["Uuid", "Created", "Version", "Name", "Description", "IsEnabled"],
                UpdateProperties = ["Uuid", "Created", "Version", "Name", "Description", "IsEnabled"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", false, true),
                    ["IsEnabled"] = new("boolean", false, true),
                }),
            })
            .For(connection);
}
