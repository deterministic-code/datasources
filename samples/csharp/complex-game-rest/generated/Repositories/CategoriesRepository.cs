using Datasources;

public sealed class CategoriesRepository : CRUDRepository<Backend.Types.View.Category>
{

    public CategoriesRepository(IConnection connection) : base(connection, Options(connection))
    {
        AddEnriched(new("Parent", "CategoryId", "Id"), new CRUDRepository<Backend.Types.View.Category>(connection, Options(connection)));
    }

    public static IReadonlyRepository<Backend.Types.View.Category> CreateReadonly(IConnection connection)
    {
        var repo = new ReadonlyRepository<Backend.Types.View.Category>(connection, Options(connection));
        repo.AddEnriched(new("Parent", "CategoryId", "Id"), new ReadonlyRepository<Backend.Types.View.Category>(connection, Options(connection)));
        return repo;
    }

    public static CRUDRepositoryOptions<Backend.Types.View.Category> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.Category>()
            .Add(new()
            {
                Entity = "Category",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Category"] = "categories" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                    ["IsDeleted"] = "is_deleted",
                    ["CategoryId"] = "category_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description", "IsEnabled", "IsDeleted", "CategoryId"],
                CreateProperties = ["Name", "Description", "IsEnabled", "IsDeleted"],
                UpdateProperties = ["Uuid", "Name", "Description", "IsEnabled", "IsDeleted"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", false, false),
                    ["IsEnabled"] = new("boolean", false, true),
                    ["IsDeleted"] = new("boolean", false, true),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "Category",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Category"] = "categories" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                    ["IsDeleted"] = "is_deleted",
                    ["CategoryId"] = "category_id",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description", "IsEnabled", "IsDeleted", "CategoryId"],
                CreateProperties = ["Uuid", "Created", "Version", "Name", "Description", "IsEnabled", "IsDeleted"],
                UpdateProperties = ["Uuid", "Created", "Version", "Name", "Description", "IsEnabled", "IsDeleted"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", false, false),
                    ["IsEnabled"] = new("boolean", false, true),
                    ["IsDeleted"] = new("boolean", false, true),
                }),
            })
            .For(connection);
}
