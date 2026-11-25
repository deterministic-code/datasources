using Datasources;

public sealed class BadgeNodesRepository : CRUDRepository<Backend.Types.Datasource.BadgeNode>
{
    public BadgeNodesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.BadgeNode> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.BadgeNode>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.BadgeNode> Options() => new()
    {
        Entity = "BadgeNode",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["BadgeNode"] = "badge_nodes" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentBadgeId"] = "parent_badge_id",
            ["ParentCode"] = "parent_code",
            ["Name"] = "name",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentBadgeId", "ParentCode", "Name"],
        CreateProperties = ["ParentBadgeId", "ParentCode", "Name"],
        UpdateProperties = ["ParentBadgeId", "ParentCode", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentBadgeId"] = new("uuid", false, false),
            ["ParentCode"] = new("string", false, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
