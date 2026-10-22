using Datasources;

public sealed class ContactPreferencesRepository : CRUDRepository<Backend.Types.Datasource.ContactPreferences>
{
    public ContactPreferencesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.ContactPreferences> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.ContactPreferences>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.Datasource.ContactPreferences> Options() => new()
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
    };
}
