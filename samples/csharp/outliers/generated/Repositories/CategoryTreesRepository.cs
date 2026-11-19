using Datasources;

public sealed class CategoryTreesRepository : ReadonlyRepository<Backend.Types.View.CategoryTree>
{

    public CategoryTreesRepository(IConnection connection) : base(connection, Options())
    {
        AddEnriched(new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List), new ReadonlyRepository<Backend.Types.View.CategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.CategoryTree>());
    }

    public static IReadonlyRepository<Backend.Types.View.CategoryTree> CreateReadonly(IConnection connection) =>
        new CategoryTreesRepository(connection);

    public static ReadonlyRepositoryMappingOptions<Backend.Types.View.CategoryTree> Options() => new()
    {
        Entity = "CategoryTree",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["CategoryTree"] = "categories" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentCategoryId"] = "parent_category_id",
            ["Name"] = "name",
        }),
        ReadProperties = ["Id", "ParentCategoryId", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentCategoryId"] = new("integer", true, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
