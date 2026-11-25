using Datasources;

public sealed class BadgeSettingsRepository : CRUDRepository<Backend.Types.Datasource.BadgeSetting>
{
    public BadgeSettingsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.BadgeSetting> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.BadgeSetting>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.BadgeSetting> Options() => new()
    {
        Entity = "BadgeSetting",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["BadgeSetting"] = "badge_settings" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentBadgeId"] = "parent_badge_id",
            ["ParentCode"] = "parent_code",
            ["Name"] = "name",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentBadgeId", "ParentCode", "Name", "Value"],
        CreateProperties = ["ParentBadgeId", "ParentCode", "Name", "Value"],
        UpdateProperties = ["ParentBadgeId", "ParentCode", "Name", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentBadgeId"] = new("uuid", false, false),
            ["ParentCode"] = new("string", false, false),
            ["Name"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
