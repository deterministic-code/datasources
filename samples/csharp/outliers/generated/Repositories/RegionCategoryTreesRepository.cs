using Datasources;

public sealed class RegionCategoryTreesRepository : ReadonlyRepository<Backend.Types.View.RegionCategoryTree>
{

    public RegionCategoryTreesRepository(IConnection connection) : base(connection, Options())
    {
        AddEnriched(new("Children", "RegionId", "ParentRegionId", Keys: [new EnrichmentMatch("RegionId", "ParentRegionId"), new EnrichmentMatch("CategoryId", "ParentCategoryId")], Type: EnrichmentPropertyType.List), new ReadonlyRepository<Backend.Types.View.RegionCategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.RegionCategoryTree>());
    }

    public static IReadonlyRepository<Backend.Types.View.RegionCategoryTree> CreateReadonly(IConnection connection) =>
        new RegionCategoryTreesRepository(connection);

    public static ReadonlyRepositoryMappingOptions<Backend.Types.View.RegionCategoryTree> Options() => new()
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
        ReadProperties = ["RegionId", "CategoryId", "ParentRegionId", "ParentCategoryId", "Name"],
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
