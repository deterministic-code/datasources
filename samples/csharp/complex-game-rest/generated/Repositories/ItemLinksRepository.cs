using Datasources;

public sealed class ItemLinksRepository : CRUDRepository<Backend.Types.Datasource.ItemLink>
{
    public ItemLinksRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ItemLink> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ItemLink>(connection, Options(connection));

    public static CRUDRepositoryOptions<Backend.Types.Datasource.ItemLink> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.ItemLink>()
            .Add(new()
            {
                Entity = "ItemLink",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ItemLink"] = "item_links" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FromItemId"] = "from_item_id",
                    ["ToItemId"] = "to_item_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FromItemId", "ToItemId"],
                CreateProperties = ["FromItemId", "ToItemId"],
                UpdateProperties = ["Uuid", "FromItemId", "ToItemId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FromItemId"] = new("biginteger", false, false),
                    ["ToItemId"] = new("biginteger", false, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "ItemLink",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ItemLink"] = "item_links" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["FromItemId"] = "from_item_id",
                    ["ToItemId"] = "to_item_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "FromItemId", "ToItemId"],
                CreateProperties = ["Uuid", "Created", "Version", "FromItemId", "ToItemId"],
                UpdateProperties = ["Uuid", "Created", "Version", "FromItemId", "ToItemId"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["FromItemId"] = new("biginteger", false, false),
                    ["ToItemId"] = new("biginteger", false, false),
                }),
            })
            .For(connection);
}
