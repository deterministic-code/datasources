using Datasources;

public sealed class LocalePreferencesRepository : CRUDRepository<Backend.Types.View.LocalePreference>
{
    public LocalePreferencesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.LocalePreference> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.LocalePreference>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.LocalePreference> Options() => new()
    {
        Entity = "LocalePreference",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["LocalePreference"] = "locale_preferences" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Locale"] = "locale",
            ["Timezone"] = "timezone",
        }),
        Ids = [],
        ReadProperties = ["Locale", "Timezone"],
        CreateProperties = ["Locale", "Timezone"],
        UpdateProperties = ["Locale", "Timezone"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Locale"] = new("string", false, false),
            ["Timezone"] = new("string", false, false),
        }),
    };
}
