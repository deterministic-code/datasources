using Datasources;

public sealed class ContactPreferencesRepository : CRUDRepository<Backend.Types.Datasource.ContactPreferences>
{
    public ContactPreferencesRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactPreferences> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactPreferences>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactPreferences> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.Datasource.ContactPreferences>()
            .Add(new()
            {
                Entity = "ContactPreferences",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactPreferences"] = "contact_preferences" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Locale"] = "locale",
                    ["Timezone"] = "timezone",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Locale", "Timezone"],
                CreateProperties = ["Uuid", "Version", "ContactId", "Key", "Locale", "Timezone"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Locale", "Timezone"],
            })
            .Add("mysql", new()
            {
                Entity = "ContactPreferences",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactPreferences"] = "contact_preferences" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Locale"] = "locale",
                    ["Timezone"] = "timezone",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Locale", "Timezone"],
                CreateProperties = ["ContactId", "Key", "Locale", "Timezone"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Locale", "Timezone"],
            })
            .Add("postgres", new()
            {
                Entity = "ContactPreferences",
                EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactPreferences"] = "contact_preferences" }),
                PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["Id"] = "id",
                    ["Uuid"] = "uuid",
                    ["Created"] = "created",
                    ["Updated"] = "updated",
                    ["Version"] = "version",
                    ["ContactId"] = "contact_id",
                    ["Key"] = "key",
                    ["Locale"] = "locale",
                    ["Timezone"] = "timezone",
                }),
                Ids = ["Id"],
                ReadProperties = ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "Key", "Locale", "Timezone"],
                CreateProperties = ["ContactId", "Key", "Locale", "Timezone"],
                UpdateProperties = ["Uuid", "Version", "ContactId", "Key", "Locale", "Timezone"],
            })
            .For(connection);
}
