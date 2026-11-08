using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class ForeignKeyTypeIntegrationTests
{
    public ForeignKeyTypeIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class LongContact
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class LongAddress
    {
        public long Id { get; set; }
        public long ContactId { get; set; }
        public string City { get; set; } = "";
    }

    sealed class GuidContact
    {
        public Guid Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class GuidAddress
    {
        public long Id { get; set; }
        public Guid ContactId { get; set; }
        public string City { get; set; } = "";
    }

    [Fact]
    public async Task ForeignKeyMatchesLongParentId()
    {
        var (contactsRepo, addressesRepo) = await Repos<LongContact, LongAddress>(
            """
            CREATE TABLE contacts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);
            CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id INTEGER NOT NULL, city TEXT NOT NULL);
            """,
            new("integer"));
        var parent = await contactsRepo.Create(new LongContact { Name = "Ada" });
        var child = await addressesRepo.Create(new LongAddress { ContactId = parent.Id, City = "London" });
        var loaded = await addressesRepo.ReadSingleById(child.Id);
        Assert.IsType<long>(parent.Id);
        Assert.IsType<long>(child.ContactId);
        Assert.Equal(parent.Id, child.ContactId);
        Assert.Equal(parent.Id, loaded!.ContactId);
    }

    [Fact]
    public async Task ForeignKeyMatchesGuidParentId()
    {
        var (contactsRepo, addressesRepo) = await Repos<GuidContact, GuidAddress>(
            """
            CREATE TABLE contacts (id TEXT PRIMARY KEY, name TEXT NOT NULL);
            CREATE TABLE addresses (id INTEGER PRIMARY KEY AUTOINCREMENT, contact_id TEXT NOT NULL, city TEXT NOT NULL);
            """,
            new("uuid"),
            create: ["Id", "Name"]);
        var parent = await contactsRepo.Create(new GuidContact
        {
            Id = Guid.Parse("11111111-1111-1111-1111-111111111111"),
            Name = "Ada",
        });
        var child = await addressesRepo.Create(new GuidAddress { ContactId = parent.Id, City = "London" });
        var loaded = await addressesRepo.ReadSingleById(child.Id);
        Assert.IsType<Guid>(parent.Id);
        Assert.IsType<Guid>(child.ContactId);
        Assert.Equal(parent.Id, child.ContactId);
        Assert.Equal(parent.Id, loaded!.ContactId);
    }

    static async Task<(ICRUDRepository<TParent> contacts, ICRUDRepository<TChild> addresses)> Repos<TParent, TChild>(
        string sql,
        SchemaColumn id,
        string[]? create = null)
        where TParent : new()
        where TChild : new()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await Execute(connection, sql);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        return (
            ds.CreateRepository<TParent>(connection, new CRUDRepositoryOptions<TParent>
            {
                Entity = "contacts",
                PropertyMappings = casing,
                Ids = ["Id"],
                Schema = new Schema(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = id with { IsId = true },
                    ["Name"] = new("string"),
                }),
                CreateProperties = create,
            }),
            ds.CreateRepository<TChild>(connection, new CRUDRepositoryOptions<TChild>
            {
                Entity = "addresses",
                PropertyMappings = casing,
                Ids = ["Id"],
                Schema = new Schema(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", IsId: true),
                    ["ContactId"] = id,
                    ["City"] = new("string"),
                }),
            }));
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}
