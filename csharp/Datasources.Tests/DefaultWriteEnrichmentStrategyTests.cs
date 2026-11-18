using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class DefaultWriteEnrichmentStrategyTests
{
    sealed class Parent
    {
        public long Id { get; set; }
        public string TypeName { get; set; } = "";
        public List<Child> Children { get; set; } = [];
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
    }

    sealed class UserType
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    static string Column(string property) => property == "Type" ? "type_id" : property;

    [Fact]
    public void SourcesIncludeObjectForeignKeys()
    {
        var (write, tree) = Graph();
        var sources = write.Sources(tree, Column);
        Assert.Contains("Type", sources);
        Assert.Contains("type_id", sources);
        Assert.DoesNotContain("Children", sources);
        Assert.DoesNotContain("Id", sources);
    }

    [Fact]
    public void DestinationIsNestedProperty()
    {
        var (write, tree) = Graph();
        Assert.True(write.Destination(tree, "TypeName"));
        Assert.True(write.Destination(tree, "Children"));
        Assert.False(write.Destination(tree, "Type"));
    }

    [Fact]
    public async Task LookupRemapsColumnOntoSource()
    {
        var (write, tree) = Graph();
        var record = new Row { ["type_id"] = 7 };
        await write.Lookup(tree, [new Parent()], record, new Schema(new Dictionary<string, SchemaColumn>()), Column);
        Assert.Equal(new Value.Int(7), record["type_id"]);
        Assert.False(record.ContainsKey("Type"));
    }

    static (DefaultWriteEnrichmentStrategy<Parent> Write, IRepositoryTree Tree) Graph()
    {
        var connection = new MemoryDriver().Connection(null);
        var typesRepo = Open<UserType>(connection);
        var childrenRepo = Open<Child>(connection);
        var parentsRepo = Open<Parent>(connection);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        return (new DefaultWriteEnrichmentStrategy<Parent>(), new RepositoryTree(new RepositoryTreeNode(parentsRepo, "")));
    }

    static CRUDRepository<T> Open<T>(IConnection connection) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T> { Ids = ["Id"] });
}
