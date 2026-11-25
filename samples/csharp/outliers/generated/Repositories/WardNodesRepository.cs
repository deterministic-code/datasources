using Datasources;

public sealed class WardNodesRepository : CRUDRepository<Backend.Types.Datasource.WardNode>
{
    public WardNodesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.WardNode> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.WardNode>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.WardNode> Options() => new()
    {
        Entity = "WardNode",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["WardNode"] = "ward_nodes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentRegionId"] = "parent_region_id",
            ["ParentWardId"] = "parent_ward_id",
            ["Name"] = "name",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentRegionId", "ParentWardId", "Name"],
        CreateProperties = ["ParentRegionId", "ParentWardId", "Name"],
        UpdateProperties = ["ParentRegionId", "ParentWardId", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentRegionId"] = new("integer", false, false),
            ["ParentWardId"] = new("integer", false, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
