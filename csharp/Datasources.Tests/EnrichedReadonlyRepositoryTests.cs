using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class ReadonlyRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
        public int type_id { get; set; }
        public string type_name { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    sealed class TypeRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class Child
    {
        public int parent_id { get; set; }
        public int type_id { get; set; }
        public string label { get; set; } = "";
        public string type_name { get; set; } = "";
    }

    sealed class View
    {
        public string UserType { get; set; } = "";
    }

    [Fact]
    public Task ReadFillsLookupSelectAndLeavesMissingDefault() =>
        FillLookup(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetFillsLookupSelectAndLeavesMissingDefault() =>
        FillLookup(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task FillLookup(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parentsRepo = new ReadonlyRepository<Parent>(await Open(
            new Row { ["id"] = 1, ["type_id"] = 10 },
            new Row { ["id"] = 2, ["type_id"] = 99 }));
        var typesRepo = new ReadonlyRepository<TypeRow>(await Open(new Row { ["id"] = 10, ["name"] = "person" }));
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo, strategy);
        var items = new List<Parent>();
        await foreach (var item in parentsRepo.Read())
            items.Add(item);
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public Task ReadFillsListsAndReadSingleIsEnriched() =>
        FillLists(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetFillsListsAndReadSingleIsEnriched() =>
        FillLists(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task FillLists(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parentsRepo = new ReadonlyRepository<Parent>(await Open(new Row { ["id"] = 1, ["type_id"] = 10 }));
        var typesRepo = new ReadonlyRepository<TypeRow>(await Open(new Row { ["id"] = 10, ["name"] = "org" }));
        var childrenRepo = new ReadonlyRepository<Child>(await Open(
            new Row { ["parent_id"] = 1, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["label"] = "b" },
            new Row { ["parent_id"] = 2, ["label"] = "x" }));
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo, strategy);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo, strategy);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal(["a", "b"], item.children.Select(child => child.label));
    }

    [Fact]
    public Task ReadNestsListLookupAndKeepsRootSibling() =>
        NestListLookup(new DefaultReadonlyEnrichmentStrategy<Parent>());

    [Fact]
    public Task HashsetNestsListLookupAndKeepsRootSibling() =>
        NestListLookup(new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());

    async Task NestListLookup(IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var parentsRepo = new ReadonlyRepository<Parent>(await Open(new Row { ["id"] = 1, ["type_id"] = 10 }));
        var typesRepo = new ReadonlyRepository<TypeRow>(await Open(new Row { ["id"] = 10, ["name"] = "org" }));
        var childTypesRepo = new ReadonlyRepository<TypeRow>(await Open(new Row { ["id"] = 3, ["name"] = "home" }));
        var childrenRepo = new ReadonlyRepository<Child>(await Open(
            new Row { ["parent_id"] = 1, ["type_id"] = 3, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["type_id"] = 9, ["label"] = "b" }));
        childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo, strategy);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo, strategy);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal("home", item.children[0].type_name);
        Assert.Equal("", item.children[1].type_name);
    }

    [Fact]
    public async Task FlatToNestedCollapsesJoinRows()
    {
        Row?[] rows =
        [
            new() { ["id"] = 1, ["type_name"] = "person" },
            new() { ["id"] = 2, ["type_name"] = "org" }
        ];
        IList<object> parents = [];
        var parentsRepo = new ReadonlyRepository<Parent>(new MemoryProvider().Connection(null));
        await new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
        {
            Key = "id",
            Fields = new Dictionary<string, TypeMapField> { ["id"] = "id", ["type_name"] = "type_name" }
        }).Apply(parentsRepo, new("type_name", "id", "id"), parents, rows);
        var items = parents.Cast<Parent>();
        Assert.Equal([1, 2], items.Select(item => item.id));
        Assert.Equal(["person", "org"], items.Select(item => item.type_name));
    }

    [Fact]
    public async Task LookupKeyComesFromRenamedRow()
    {
        var typesRepo = new ReadonlyRepository<TypeRow>(await Open(new Row { ["id"] = 1, ["name"] = "admin" }));
        var usersRepo = new ReadonlyRepository<View>(
            await Open(new Row { ["user_type_id"] = 1 }),
            new ReadonlyRepositoryOptions<View>
            {
                ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
                {
                    ["UserType"] = "user_type_id",
                }),
            });
        usersRepo.AddEnriched(new("UserType", "UserType", "id", "name"), typesRepo);
        var item = await usersRepo.ReadSingle();
        Assert.Equal("admin", item!.UserType);
    }

    static async Task<IConnection> Open(params Row[] rows)
    {
        var connection = new MemoryProvider().Connection(null);
        await connection.Open();
        foreach (var row in rows)
            await connection.CreateCommand().Execute(
                "insert",
                [.. row.Select(pair => new Parameter(pair.Key, pair.Value))]);
        return connection;
    }
}
