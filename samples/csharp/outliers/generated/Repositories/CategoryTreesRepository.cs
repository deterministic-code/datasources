using Datasources;

public sealed class CategoryTreesRepository : CRUDRepository<Backend.Types.View.CategoryTree>
{

    public CategoryTreesRepository(IConnection connection) : base(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<Backend.Types.View.CategoryTree>(create: new TreeCreateEnrichmentStrategy<Backend.Types.View.CategoryTree>()) })
    {
        AddEnriched(new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List), new CRUDRepository<Backend.Types.View.CategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.CategoryTree>());
    }

    public static IReadonlyRepository<Backend.Types.View.CategoryTree> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.CategoryTree>(connection, Options());
        repo.AddEnriched(new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List), new ReadonlyRepository<Backend.Types.View.CategoryTree>(connection, Options()), new TreeReadonlyEnrichmentStrategy<Backend.Types.View.CategoryTree>());
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.CategoryTree> Options() => new()
    {
        Entity = "CategoryTree",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["CategoryTree"] = "categories" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentCategoryId"] = "parent_category_id",
            ["Name"] = "name",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentCategoryId", "Name"],
        CreateProperties = ["ParentCategoryId", "Name"],
        UpdateProperties = ["ParentCategoryId", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentCategoryId"] = new("integer", true, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
