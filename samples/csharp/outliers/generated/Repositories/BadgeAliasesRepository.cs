using Datasources;

public sealed class BadgeAliasesRepository : CRUDRepository<Backend.Types.Datasource.BadgeAlias>
{
    public BadgeAliasesRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.Datasource.BadgeAlias> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.Datasource.BadgeAlias>(connection, Options());

    public static CRUDRepositoryOptions<Backend.Types.Datasource.BadgeAlias> Options() => new()
    {
        Entity = "BadgeAlias",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["BadgeAlias"] = "badge_aliases" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["ParentBadgeId"] = "parent_badge_id",
            ["ParentCode"] = "parent_code",
            ["Name"] = "name",
        }),
        Ids = ["Id"],
        ReadProperties = ["Id", "ParentBadgeId", "ParentCode", "Name"],
        CreateProperties = ["ParentBadgeId", "ParentCode", "Name"],
        UpdateProperties = ["ParentBadgeId", "ParentCode", "Name"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["ParentBadgeId"] = new("uuid", false, false),
            ["ParentCode"] = new("string", false, false),
            ["Name"] = new("string", false, false),
        }),
    };
}
