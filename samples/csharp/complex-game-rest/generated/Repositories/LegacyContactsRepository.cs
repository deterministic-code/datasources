using Datasources;

public sealed class LegacyContactsRepository : CRUDRepository<Backend.Types.View.LegacyContact>
{
    public LegacyContactsRepository(IConnection connection) : base(connection, Options()) { }

    public static IReadonlyRepository<Backend.Types.View.LegacyContact> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.LegacyContact>(connection, Options());

    static CRUDRepositoryOptions<Backend.Types.View.LegacyContact> Options() => new()
    {
        Entity = "LegacyContact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["LegacyContact"] = "legacy_contacts" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Key"] = "key",
            ["FirstName"] = "first_name",
            ["LastName"] = "last_name",
            ["Email"] = "email",
            ["ImportedAt"] = "imported_at",
        }),
        Ids = [],
        ReadProperties = ["Key", "FirstName", "LastName", "Email", "ImportedAt"],
        CreateProperties = ["Key", "FirstName", "LastName", "Email", "ImportedAt"],
        UpdateProperties = ["Key", "FirstName", "LastName", "Email", "ImportedAt"],
        Schema = new(new Dictionary<string, SchemaColumn>
        {
            ["Key"] = new("string", false, false),
            ["FirstName"] = new("string", false, false),
            ["LastName"] = new("string", false, false),
            ["Email"] = new("string", true, false),
            ["ImportedAt"] = new("datetime", true, false),
        }),
    };
}
