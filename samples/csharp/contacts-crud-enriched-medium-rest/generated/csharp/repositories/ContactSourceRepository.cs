using Datasources;

public sealed class ContactSourceRepository : CRUDRepository<Backend.Types.Datasource.ContactSource>
{
    public ContactSourceRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactSource> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactSource>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactSource> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.ContactSource>()
            .Add(new()
            {
                Entity = "ContactSource",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSource"] = "contact_sources" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Created", "Updated", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("sqlite", new()
            {
                Entity = "ContactSource",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSource"] = "contact_sources" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Uuid", "Version", "Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("mysql", new()
            {
                Entity = "ContactSource",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSource"] = "contact_sources" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .Add("postgres", new()
            {
                Entity = "ContactSource",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSource"] = "contact_sources" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["Name"] = "name",
                    ["Description"] = "description",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "Name", "Description"],
                CreateProperties = ["Name", "Description"],
                UpdateProperties = ["Uuid", "Version", "Name", "Description"],
                Schema = new(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", false, false),
                    ["Uuid"] = new("uuid", false, true),
                    ["Created"] = new("datetime", false, true),
                    ["Updated"] = new("datetime", false, true),
                    ["Version"] = new("binary", false, true),
                    ["Name"] = new("string", false, false),
                    ["Description"] = new("string", true, false),
                }),
            })
            .For(connection);
}
