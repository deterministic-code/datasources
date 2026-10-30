using Datasources;

public sealed class LegacyContactRepository : CRUDRepository<Backend.Types.View.LegacyContact>
{
    public LegacyContactRepository(IConnection connection) : base(connection, Options(connection)) { }

    public static IReadonlyRepository<Backend.Types.View.LegacyContact> CreateReadonly(IConnection connection) =>
        new ReadonlyRepository<Backend.Types.View.LegacyContact>(connection, Options(connection));

    static CRUDRepositoryOptions<Backend.Types.View.LegacyContact> Options(IConnection connection) =>
        new CRUDRepositoryBuilder<Backend.Types.View.LegacyContact>()
            .Add(new()
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
            })
            .Add("sqlite", new()
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
            })
            .Add("mysql", new()
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
            })
            .Add("postgres", new()
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
            })
            .For(connection);
}
