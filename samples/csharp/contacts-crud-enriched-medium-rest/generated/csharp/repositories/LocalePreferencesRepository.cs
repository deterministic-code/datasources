using Datasources;

public sealed class LocalePreferencesRepository : CRUDRepository<Backend.Types.View.LocalePreferences>
{
    public LocalePreferencesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.LocalePreferences> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.LocalePreferences>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.LocalePreferences> Options() => new()
    {
        Entity = "LocalePreferences",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["LocalePreferences"] = "locale_preferences" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Locale"] = "locale",
            ["Timezone"] = "timezone",
        }),
        Ids = [],
        ReadProperties = ["Locale", "Timezone"],
        CreateProperties = ["Locale", "Timezone"],
        UpdateProperties = ["Locale", "Timezone"],
    };
}
