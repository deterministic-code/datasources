using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class ObjectEnrichmentRoundtripTests
{
    public ObjectEnrichmentRoundtripTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class Kind
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class User
    {
        public int Id { get; set; }
        public Guid Uuid { get; set; }
        public bool IsEnabled { get; set; }
        public int Type { get; set; }
        public string TypeName { get; set; } = "";
        public string Username { get; set; } = "";
    }

    [Fact]
    public async Task CreateAndReadRoundTripBoolAndGuidWithObjectEnrichment()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        foreach (var sql in new[]
        {
            "CREATE TABLE types (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)",
            """
            CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid TEXT,
                is_enabled INTEGER NOT NULL,
                type_id INTEGER NOT NULL,
                username TEXT NOT NULL
            )
            """,
        })
            await command.Execute(new Native(sql));
        var mappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Id"] = "id",
            ["Uuid"] = "uuid",
            ["IsEnabled"] = "is_enabled",
            ["Type"] = "type_id",
            ["Username"] = "username",
            ["Name"] = "name",
        });
        var typesRepo = new CRUDRepository<Kind>(connection, new CRUDRepositoryOptions<Kind>
        {
            Entity = "types",
            PropertyMappings = mappings,
            Ids = ["Id"],
            Schema = new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("integer", IsId: true),
                ["Name"] = new("string"),
            }),
        });
        var usersRepo = new CRUDRepository<User>(connection, new CRUDRepositoryOptions<User>
        {
            Entity = "users",
            PropertyMappings = mappings,
            Ids = ["Id"],
            ReadProperties = ["Id", "Uuid", "IsEnabled", "Type", "Username"],
            Schema = new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("integer", IsId: true),
                ["Uuid"] = new("uuid"),
                ["IsEnabled"] = new("boolean"),
                ["Type"] = new("integer"),
                ["Username"] = new("string"),
            }),
        });
        usersRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        var kind = await typesRepo.Create(new Kind { Name = "admin" });
        var id = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var created = await usersRepo.Create(new User
        {
            Username = "ada",
            IsEnabled = true,
            Uuid = id,
            Type = kind.Id,
        });
        var loaded = await usersRepo.ReadSingleById(created.Id);
        Assert.True(created.IsEnabled);
        Assert.Equal(id, created.Uuid);
        Assert.True(loaded!.IsEnabled);
        Assert.Equal(id, loaded.Uuid);
        Assert.Equal(kind.Id, loaded.Type);
    }
}
