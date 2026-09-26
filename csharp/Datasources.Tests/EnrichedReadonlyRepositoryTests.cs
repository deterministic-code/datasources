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
    public async Task ReadFillsLookupSelectAndLeavesMissingDefault()
    {
        var connection = await Setup(
            new Row { ["id"] = 1, ["type_id"] = 10 },
            new Row { ["id"] = 2, ["type_id"] = 99 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "person" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public async Task HashsetFillsLookupSelectAndLeavesMissingDefault()
    {
        var connection = await Setup(
            new Row { ["id"] = 1, ["type_id"] = 10 },
            new Row { ["id"] = 2, ["type_id"] = 99 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "person" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        parentsRepo.AddEnriched(
            new("type_name", "type_id", "id", "name"),
            typesRepo,
            new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public async Task ReadFillsListsAndReadSingleIsEnriched()
    {
        var connection = await Setup(new Row { ["id"] = 1, ["type_id"] = 10 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "org" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = 1, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["label"] = "b" },
            new Row { ["parent_id"] = 2, ["label"] = "x" });
        var childrenRepo = new ReadonlyRepository<Child>(childrenConnection);
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal(["a", "b"], item.children.Select(child => child.label));
    }

    [Fact]
    public async Task HashsetFillsListsAndReadSingleIsEnriched()
    {
        var connection = await Setup(new Row { ["id"] = 1, ["type_id"] = 10 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "org" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = 1, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["label"] = "b" },
            new Row { ["parent_id"] = 2, ["label"] = "x" });
        var childrenRepo = new ReadonlyRepository<Child>(childrenConnection);
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo, strategy);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo, strategy);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal(["a", "b"], item.children.Select(child => child.label));
    }

    [Fact]
    public async Task ReadNestsListLookupAndKeepsRootSibling()
    {
        var connection = await Setup(new Row { ["id"] = 1, ["type_id"] = 10 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "org" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childTypesConnection = await Setup(new Row { ["id"] = 3, ["name"] = "home" });
        var childTypesRepo = new ReadonlyRepository<TypeRow>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = 1, ["type_id"] = 3, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["type_id"] = 9, ["label"] = "b" });
        var childrenRepo = new ReadonlyRepository<Child>(childrenConnection);
        childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal("home", item.children[0].type_name);
        Assert.Equal("", item.children[1].type_name);
    }

    [Fact]
    public async Task HashsetNestsListLookupAndKeepsRootSibling()
    {
        var connection = await Setup(new Row { ["id"] = 1, ["type_id"] = 10 });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesConnection = await Setup(new Row { ["id"] = 10, ["name"] = "org" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childTypesConnection = await Setup(new Row { ["id"] = 3, ["name"] = "home" });
        var childTypesRepo = new ReadonlyRepository<TypeRow>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = 1, ["type_id"] = 3, ["label"] = "a" },
            new Row { ["parent_id"] = 1, ["type_id"] = 9, ["label"] = "b" });
        var childrenRepo = new ReadonlyRepository<Child>(childrenConnection);
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
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
        var typesConnection = await Setup(new Row { ["id"] = 1, ["name"] = "admin" });
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var usersConnection = await Setup(new Row { ["user_type_id"] = 1 });
        var usersRepo = new ReadonlyRepository<View>(usersConnection, new ReadonlyRepositoryOptions<View>
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

    static async Task<IConnection> Setup(params Row[] rows)
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
