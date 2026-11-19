using Datasources;

public sealed class NodeEdgesRepository : CRUDRepository<Backend.Types.Datasource.NodeEdge>
{
    public NodeEdgesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.NodeEdge> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.NodeEdge>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.NodeEdge> Options() => new()
    {
        Entity = "NodeEdge",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["NodeEdge"] = "node_edges" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["FromNodeId"] = "from_node_id",
            ["ToNodeId"] = "to_node_id",
        }),
        Ids = [],
        ReadProperties = ["FromNodeId", "ToNodeId"],
        CreateProperties = ["FromNodeId", "ToNodeId"],
        UpdateProperties = ["FromNodeId", "ToNodeId"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["FromNodeId"] = new("integer", false, false),
            ["ToNodeId"] = new("integer", false, false),
        }),
    };
}
