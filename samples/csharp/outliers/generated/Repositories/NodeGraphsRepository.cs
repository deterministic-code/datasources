using Datasources;

public sealed class NodeGraphsRepository : CRUDRepository<Backend.Types.View.NodeGraph>
{
    public NodeEdgesRepository NodeEdge { get; }

    public NodeGraphsRepository(IConnection connection) : base(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<Backend.Types.View.NodeGraph>(create: new GraphCreateEnrichmentStrategy<Backend.Types.View.NodeGraph, Backend.Types.Datasource.NodeEdge>(new NodeEdgesRepository(connection))) })
    {
        NodeEdge = new NodeEdgesRepository(connection);
        AddEnriched(new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")), new CRUDRepository<Backend.Types.View.NodeGraph>(connection, Options()), new GraphReadonlyEnrichmentStrategy<Backend.Types.View.NodeGraph, Backend.Types.Datasource.NodeEdge>(NodeEdge));
    }

    public static IReadonlyRepository<Backend.Types.View.NodeGraph> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.NodeGraph>(connection, Options());
        var NodeEdge = new NodeEdgesRepository(connection);
        repo.AddEnriched(new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")), new ReadonlyRepository<Backend.Types.View.NodeGraph>(connection, Options()), new GraphReadonlyEnrichmentStrategy<Backend.Types.View.NodeGraph, Backend.Types.Datasource.NodeEdge>(NodeEdge));
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.NodeGraph> Options() => new()
    {
        Entity = "NodeGraph",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["NodeGraph"] = "nodes" }),
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
