using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class BulkWriteEnrichmentIntegrationTests
{
    public BulkWriteEnrichmentIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class Parent
    {
        public long Id { get; set; }
        public string Label { get; set; } = "";
        public List<Child> Children { get; set; } = [];
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Label { get; set; } = "";
    }

    [Fact]
    public async Task BatchCreateStampsGeneratedParentIdOntoChildren()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await Execute(connection, """
            CREATE TABLE parents (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL);
            CREATE TABLE children (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, label TEXT NOT NULL);
            """);
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var childrenRepo = new BulkCRUDRepository<Child>(
            connection,
            new CRUDRepositoryOptions<Child>
            {
                Entity = "children",
                PropertyMappings = casing,
                Ids = ["Id"],
                Schema = new Schema(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", IsId: true),
                    ["ParentId"] = new("integer"),
                    ["Label"] = new("string"),
                }),
            },
            new BatchCommandIBulkWriteStrategy<Child>());
        var parentsRepo = new BulkCRUDRepository<Parent>(
            connection,
            new CRUDRepositoryOptions<Parent>
            {
                Entity = "parents",
                PropertyMappings = casing,
                Ids = ["Id"],
                Schema = new Schema(new Dictionary<string, SchemaColumn>
                {
                    ["Id"] = new("integer", IsId: true),
                    ["Label"] = new("string"),
                }),
            },
            new BatchCommandIBulkWriteStrategy<Parent>());
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var created = await parentsRepo.Create(
        [
            new Parent { Label = "one", Children = [new Child { Label = "a" }] },
            new Parent { Label = "two", Children = [new Child { Label = "b" }, new Child { Label = "c" }] },
        ]);
        Assert.Equal(2, created.Count);
        Assert.All(created, parent => Assert.True(parent.Id > 0));
        Assert.Equal(2, created.Select(parent => parent.Id).Distinct().Count());
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(3, children.Count);
        Assert.Equal("a", Assert.Single(children, child => child.ParentId == created[0].Id).Label);
        Assert.Equal(
            ["b", "c"],
            children.Where(child => child.ParentId == created[1].Id).Select(child => child.Label).OrderBy(label => label).ToArray());
    }

    static async Task Execute(IConnection connection, string sql)
    {
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(new Native(statement));
    }
}
