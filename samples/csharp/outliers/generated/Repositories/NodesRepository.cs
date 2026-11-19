using Datasources;

public sealed class NodesRepository : CRUDRepository<Backend.Types.Datasource.Node>
{
    public NodesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Node> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Node>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.Node> Options() => new()
    {
        Entity = "Node",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Node"] = "nodes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "Value"],
        CreateProperties = ["Value"],
        UpdateProperties = ["Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Value"] = new("string", false, false),
        }),
    };
}
