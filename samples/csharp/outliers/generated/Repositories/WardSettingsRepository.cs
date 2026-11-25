using Datasources;

public sealed class WardSettingsRepository : CRUDRepository<Backend.Types.Datasource.WardSetting>
{
    public WardSettingsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.WardSetting> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.WardSetting>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.WardSetting> Options() => new()
    {
        Entity = "WardSetting",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["WardSetting"] = "ward_settings" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentRegionId"] = "parent_region_id",
            ["ParentWardId"] = "parent_ward_id",
            ["Name"] = "name",
            ["Value"] = "value",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentRegionId", "ParentWardId", "Name", "Value"],
        CreateProperties = ["ParentRegionId", "ParentWardId", "Name", "Value"],
        UpdateProperties = ["ParentRegionId", "ParentWardId", "Name", "Value"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentRegionId"] = new("integer", false, false),
            ["ParentWardId"] = new("integer", false, false),
            ["Name"] = new("string", false, false),
            ["Value"] = new("string", false, false),
        }),
    };
}
