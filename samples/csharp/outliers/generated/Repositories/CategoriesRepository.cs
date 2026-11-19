using Datasources;

public sealed class CategoriesRepository : CRUDRepository<Backend.Types.Datasource.Category>
{
    public CategoriesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.Category> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.Category>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.Category> Options() => new()
    {
        Entity = "Category",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Category"] = "categories" }),
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
