using Datasources;

public sealed class ItemsRepository : CRUDRepository<Backend.Types.Datasource.Item>
{
    public ItemsRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Item> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Item>(connection, Options(connection));

    public static CRUDRepositoryOptions<Backend.Types.Datasource.Item> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.Item>()
            .Add(new()
            {
                Entity = "Item",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Item"] = "items" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name"],
                CreateProperties = ["Name"],
                UpdateProperties = ["Uuid", "Name"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Item",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Item"] = "items" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name"],
                CreateProperties = ["Uuid", "Created", "Version", "Name"],
                UpdateProperties = ["Uuid", "Created", "Version", "Name"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                }),
            })
            .For(connection);
}
