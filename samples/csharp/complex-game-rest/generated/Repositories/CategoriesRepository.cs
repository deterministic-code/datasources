using Datasources;

public sealed class CategoriesRepository : CRUDRepository<Backend.Types.View.Category>
{
    public CategoriesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.View.Category> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.Category>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.View.Category> Options(IConnection connection) =>
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
                    ["CategoryId"] = "category_id",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                    ["IsDeleted"] = "is_deleted",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                CreateProperties = ["CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                UpdateProperties = ["Uuid", "CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["CategoryId"] = new("biginteger", true, false),
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
                    ["CategoryId"] = "category_id",
                    ["Name"] = "name",
                    ["Description"] = "description",
                    ["IsEnabled"] = "is_enabled",
                    ["IsDeleted"] = "is_deleted",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                CreateProperties = ["Uuid", "Created", "Version", "CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                UpdateProperties = ["Uuid", "Created", "Version", "CategoryId", "Name", "Description", "IsEnabled", "IsDeleted"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("biginteger", false, false, true),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["CategoryId"] = new("biginteger", true, false),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", false, false),
                    ["IsEnabled"] = new("boolean", false, true),
                    ["IsDeleted"] = new("boolean", false, true),
                }),
            })
            .For(connection);
}
