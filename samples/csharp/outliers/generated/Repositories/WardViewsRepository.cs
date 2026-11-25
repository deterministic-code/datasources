using Datasources;

public sealed class WardViewsRepository : CRUDRepository<Backend.Types.View.WardView>
{
    public WardAliasesRepository WardAlias { get; }
    public WardSettingsRepository WardSetting { get; }
    public WardNodesRepository WardNode { get; }

    public WardViewsRepository(IConnection connection) : base(connection, Options())
    {
        WardAlias = new WardAliasesRepository(connection);
        WardSetting = new WardSettingsRepository(connection);
        WardNode = new WardNodesRepository(connection);
        AddEnriched(new("Aliases", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardAlias);
        AddEnriched(new("Settings", "RegionId", "ParentRegionId", "Value", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Name", Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardSetting);
        AddEnriched(new("Nodes", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardNode);
    }

    public static IReadonlyRepository<Backend.Types.View.WardView> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.WardView>(connection, Options());
        var WardAlias = new WardAliasesRepository(connection);
        var WardSetting = new WardSettingsRepository(connection);
        var WardNode = new WardNodesRepository(connection);
        repo.AddEnriched(new("Aliases", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardAlias);
        repo.AddEnriched(new("Settings", "RegionId", "ParentRegionId", "Value", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Name", Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardSetting);
        repo.AddEnriched(new("Nodes", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("WardId", "ParentWardId")]), WardNode);
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.WardView> Options() => new()
    {
        Entity = "WardView",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["WardView"] = "wards" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["RegionId"] = "region_id",
            ["WardId"] = "ward_id",
            ["Name"] = "name",
        }),
        Ids = ["RegionId", "WardId"],
        ReadProperties = ["RegionId", "WardId", "Name"],
        CreateProperties = ["RegionId", "WardId", "Name"],
        UpdateProperties = ["RegionId", "WardId", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["RegionId"] = new("integer", false, false, true),
            ["WardId"] = new("integer", false, false, true),
            ["Name"] = new("string", false, false),
        }),
    };
}
