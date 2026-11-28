using Datasources;

public sealed class ItemGraphsRepository : CRUDRepository<Backend.Types.View.ItemGraph>
{
    public ItemLinksRepository ItemLinks { get; }

    public ItemGraphsRepository(IConnection connection) : base(connection, Options(connection) with { Write = new DefaultWriteEnrichmentStrategy<Backend.Types.View.ItemGraph>(create: new GraphCreateEnrichmentStrategy<Backend.Types.View.ItemGraph, Backend.Types.Datasource.ItemLink>(new ItemLinksRepository(connection))) })
    {
        ItemLinks = new ItemLinksRepository(connection);
        AddEnriched(new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromItemId", "ToItemId")), new CRUDRepository<Backend.Types.View.ItemGraph>(connection, Options(connection)), new GraphReadonlyEnrichmentStrategy<Backend.Types.View.ItemGraph, Backend.Types.Datasource.ItemLink>(ItemLinks));
    }

    public static IReadonlyRepository<Backend.Types.View.ItemGraph> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.ItemGraph>(connection, Options(connection));
        var ItemLinks = new ItemLinksRepository(connection);
        repo.AddEnriched(new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromItemId", "ToItemId")), new ReadonlyRepository<Backend.Types.View.ItemGraph>(connection, Options(connection)), new GraphReadonlyEnrichmentStrategy<Backend.Types.View.ItemGraph, Backend.Types.Datasource.ItemLink>(ItemLinks));
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.ItemGraph> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.ItemGraph>()
            .Add(new()
            {
                Entity = "ItemGraph",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ItemGraph"] = "items" }),
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
                Entity = "ItemGraph",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ItemGraph"] = "items" }),
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
