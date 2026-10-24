using Datasources;

public sealed class LegacyContactRepository : CRUDRepository<Backend.Types.View.LegacyContact>
{
    public LegacyContactRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.LegacyContact> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.LegacyContact>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.LegacyContact> Options() => new()
    {
        Entity = "LegacyContact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["LegacyContact"] = "OldContactsTbl" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Key"] = "CntID",
            ["FirstName"] = "FirstNm",
            ["LastName"] = "LastNm",
            ["Email"] = "EmailAddr",
            ["ImportedAt"] = "ImpDate",
        }),
        Ids = ["Key"],
        ReadProperties = ["Key", "FirstName", "LastName", "Email", "ImportedAt"],
        CreateProperties = ["Key", "FirstName", "LastName", "Email", "ImportedAt"],
        UpdateProperties = ["FirstName", "LastName", "Email", "ImportedAt"],
    };
}
