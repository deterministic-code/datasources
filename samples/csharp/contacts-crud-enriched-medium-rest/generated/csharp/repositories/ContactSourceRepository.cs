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
            })
            .For(connection);
}
