using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class DefaultWriteEnrichmentStrategyTests
{
    sealed class Parent
    {
        public long Id { get; set; }
        public Guid Type { get; set; }
        public bool Active { get; set; }
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
    public async Task LookupRestoresPropertyKeys()
    {
        var (write, tree) = Graph();
        var record = new Row { ["type_id"] = 7 };
        await write.Lookup(tree, [new Parent()], record, new Schema(new Dictionary<string, SchemaColumn>()), Column);
        Assert.Equal(new Value.Int(7), record["Type"]);
        Assert.False(record.ContainsKey("type_id"));
        var assigned = new Row { ["Type"] = 3 };
        await write.Lookup(tree, [new Parent()], assigned, new Schema(new Dictionary<string, SchemaColumn>()), Column);
        Assert.Equal(new Value.Int(3), assigned["Type"]);
        Assert.False(assigned.ContainsKey("type_id"));
    }

    [Fact]
    public async Task LookupThenReverseConvertConvertsBoolean()
    {
        var (write, tree) = Graph();
        var record = new Row { ["type_id"] = 7, ["Active"] = true };
        await write.Lookup(tree, [new Parent()], record, new Schema(new Dictionary<string, SchemaColumn>()), Column);
        var converter = RowTypeConverter.From(
            new Dictionary<string, TypeConverter<Value, Value>> { ["boolean"] = new Flag() },
            new Dictionary<string, string> { ["Type"] = "integer", ["Active"] = "boolean" });
        var converted = converter.ReverseConvert(record);
        Assert.Equal(new Value.Int(1), converted["Active"]);
        Assert.Equal(new Value.Int(7), converted["Type"]);
    }

    [Fact]
    public async Task LookupRowConvertsUuidAndBooleanOnPropertyKeys()
    {
        var (write, tree) = Graph();
        var id = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var item = new Parent { Type = id, Active = true };
        var objects = new MetaDataRowTypeConverter<Parent>();
        var rows = RowTypeConverter.From(
            new Dictionary<string, TypeConverter<Value, Value>>
            {
                ["boolean"] = new Flag(),
                ["uuid"] = new UuidText(),
            },
            new Dictionary<string, string> { ["Type"] = "uuid", ["Active"] = "boolean" });
        var converted = await write.LookupRow(
            tree,
            item,
            new Schema(new Dictionary<string, SchemaColumn>
            {
                ["Type"] = new("uuid"),
                ["Active"] = new("boolean"),
            }),
            Column,
            objects,
            rows);
        Assert.True(converted.ContainsKey("Type"));
        Assert.False(converted.ContainsKey("type_id"));
        Assert.Equal(new Value.Text(id.ToString()), converted["Type"]);
        Assert.Equal(new Value.Int(1), converted["Active"]);
    }

    sealed class Flag : TypeConverter<Value, Value>
    {
        public override Value Convert(Value from) => from.ToObject() is long n && n == 1;
        public override Value ReverseConvert(Value to) => to.ToObject() is true ? 1L : 0L;
    }

    sealed class UuidText : TypeConverter<Value, Value>
    {
        public override Value Convert(Value from) => from;
        public override Value ReverseConvert(Value to) =>
            to.ToObject() is Guid guid ? guid.ToString() : to;
    }

    static (IWriteEnrichmentStrategy<Parent> Write, IRepositoryTree Tree) Graph()
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
