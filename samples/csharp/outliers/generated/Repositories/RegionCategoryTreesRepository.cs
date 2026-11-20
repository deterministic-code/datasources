using Datasources;

public sealed class RegionCategoryTreesRepository : CRUDRepository<Backend.Types.View.RegionCategoryTree>
{

    public RegionCategoryTreesRepository(IConnection connection) : base(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<Backend.Types.View.RegionCategoryTree>(create: new TreeCreateEnrichmentStrategy<Backend.Types.View.RegionCategoryTree>()) })
    {
        AddEnriched(new("Children", "RegionId", "ParentRegionId", Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("CategoryId", "ParentCategoryId")], Type: EnrichmentPropertyType.List), new CRUDRepository<Backend.Types.View.RegionCategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.RegionCategoryTree>());
    }

    public static IReadonlyRepository<Backend.Types.View.RegionCategoryTree> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.RegionCategoryTree>(connection, Options());
        repo.AddEnriched(new("Children", "RegionId", "ParentRegionId", Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("CategoryId", "ParentCategoryId")], Type: EnrichmentPropertyType.List), new ReadonlyRepository<Backend.Types.View.RegionCategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.RegionCategoryTree>());
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.RegionCategoryTree> Options() => new()
    {
        Entity = "RegionCategoryTree",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["RegionCategoryTree"] = "region_categories" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["RegionId"] = "region_id",
            ["CategoryId"] = "category_id",
            ["ParentRegionId"] = "parent_region_id",
            ["ParentCategoryId"] = "parent_category_id",
            ["Name"] = "name",
        }),
        Ids = ["RegionId", "CategoryId"],
        ReadProperties = ["RegionId", "CategoryId", "ParentRegionId", "ParentCategoryId", "Name"],
        CreateProperties = ["RegionId", "CategoryId", "ParentRegionId", "ParentCategoryId", "Name"],
        UpdateProperties = ["RegionId", "CategoryId", "ParentRegionId", "ParentCategoryId", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["RegionId"] = new("integer", false, false, true),
            ["CategoryId"] = new("integer", false, false, true),
            ["ParentRegionId"] = new("integer", true, false),
            ["ParentCategoryId"] = new("integer", true, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
