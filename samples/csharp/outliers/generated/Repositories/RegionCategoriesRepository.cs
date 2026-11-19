using Datasources;

public sealed class RegionCategoriesRepository : CRUDRepository<Backend.Types.Datasource.RegionCategory>
{
    public RegionCategoriesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.RegionCategory> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.RegionCategory>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.RegionCategory> Options() => new()
    {
        Entity = "RegionCategory",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["RegionCategory"] = "region_categories" }),
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
