using Datasources;

public sealed class ContactSettingsRepository : CRUDRepository<Backend.Types.Datasource.ContactSettings>
{
    public ContactSettingsRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactSettings> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactSettings>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactSettings> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.ContactSettings>()
            .Add(new()
            {
                Entity = "ContactSettings",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSettings"] = "contact_settings" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Value"] = "value",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
                CreateProperties = ["Uuid", "Version", "ContactId", "Key", "Value"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Value"],
            })
            .Add("mysql", new()
            {
                Entity = "ContactSettings",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSettings"] = "contact_settings" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Value"] = "value",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
                CreateProperties = ["ContactId", "Key", "Value"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Value"],
            })
            .Add("postgres", new()
            {
                Entity = "ContactSettings",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSettings"] = "contact_settings" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Value"] = "value",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Value"],
                CreateProperties = ["ContactId", "Key", "Value"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Value"],
            })
            .For(connection);
}
