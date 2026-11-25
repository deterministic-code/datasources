using Datasources;

public sealed class BadgeViewsRepository : CRUDRepository<Backend.Types.View.BadgeView>
{
    public BadgeAliasesRepository BadgeAlias { get; }
    public BadgeSettingsRepository BadgeSetting { get; }
    public BadgeNodesRepository BadgeNode { get; }

    public BadgeViewsRepository(IConnection connection) : base(connection, Options())
    {
        BadgeAlias = new BadgeAliasesRepository(connection);
        BadgeSetting = new BadgeSettingsRepository(connection);
        BadgeNode = new BadgeNodesRepository(connection);
        AddEnriched(new("Aliases", "BadgeId", "ParentBadgeId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeAlias);
        AddEnriched(new("Settings", "BadgeId", "ParentBadgeId", "Value", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Name", Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeSetting);
        AddEnriched(new("Nodes", "BadgeId", "ParentBadgeId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeNode);
    }

    public static IReadonlyRepository<Backend.Types.View.BadgeView> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.BadgeView>(connection, Options());
        var BadgeAlias = new BadgeAliasesRepository(connection);
        var BadgeSetting = new BadgeSettingsRepository(connection);
        var BadgeNode = new BadgeNodesRepository(connection);
        repo.AddEnriched(new("Aliases", "BadgeId", "ParentBadgeId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeAlias);
        repo.AddEnriched(new("Settings", "BadgeId", "ParentBadgeId", "Value", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Name", Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeSetting);
        repo.AddEnriched(new("Nodes", "BadgeId", "ParentBadgeId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("BadgeId", "ParentBadgeId"), new EnrichmentMatch("Code", "ParentCode")]), BadgeNode);
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.BadgeView> Options() => new()
    {
        Entity = "BadgeView",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["BadgeView"] = "badges" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["BadgeId"] = "badge_id",
            ["Code"] = "code",
            ["Name"] = "name",
        }),
        Ids = ["BadgeId", "Code"],
        ReadProperties = ["BadgeId", "Code", "Name"],
        CreateProperties = ["BadgeId", "Code", "Name"],
        UpdateProperties = ["BadgeId", "Code", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["BadgeId"] = new("uuid", false, false, true),
            ["Code"] = new("string", false, false, true),
            ["Name"] = new("string", false, false),
        }),
    };
}
