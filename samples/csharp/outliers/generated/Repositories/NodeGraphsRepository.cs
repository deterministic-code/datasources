using Datasources;

public sealed class NodeGraphsRepository : ReadonlyRepository<Backend.Types.View.NodeGraph>
{
    public NodeEdgesRepository NodeEdge { get; }

    public NodeGraphsRepository(IConnection connection) : base(connection, Options())
    {
        NodeEdge = new NodeEdgesRepository(connection);
        AddEnriched(new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")), new ReadonlyRepository<Backend.Types.View.NodeGraph>(connection, Options()), new GraphReadonlyEnrichmentStrategy<Backend.Types.View.NodeGraph, Backend.Types.Datasource.NodeEdge>(NodeEdge));
    }

    public static IReadonlyRepository<Backend.Types.View.NodeGraph> CreateReadonly(IConnection connection) =>
        new NodeGraphsRepository(connection);

    public static ReadonlyRepositoryMappingOptions<Backend.Types.View.NodeGraph> Options() => new()
    {
        Entity = "NodeGraph",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["NodeGraph"] = "nodes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Value"] = "value",
        }),
        ReadProperties = ["Id", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Value"] = new("string", false, false),
        }),
    };
}
