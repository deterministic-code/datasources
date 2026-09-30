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
        var person = new Row { ["name"] = "person" };
        var gone = new Row { ["name"] = "gone" };
        var typesConnection = await Setup(person, gone);
        await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
        var connection = await Setup(
            new Row { ["type_id"] = person["id"] },
            new Row { ["type_id"] = gone["id"] });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("person", items[0].type_name);
        Assert.Equal("", items[1].type_name);
    }

    [Fact]
    public async Task HashsetFillsLookupSelectAndLeavesMissingDefault()
    {
        var person = new Row { ["name"] = "person" };
        var gone = new Row { ["name"] = "gone" };
        var typesConnection = await Setup(person, gone);
        await typesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
        var connection = await Setup(
            new Row { ["type_id"] = person["id"] },
            new Row { ["type_id"] = gone["id"] });
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
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
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var parent = new Row { ["type_id"] = org["id"] };
        var other = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent, other);
        await connection.CreateCommand().Execute("delete", [new Parameter("id", other["id"])]);
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["label"] = "b" },
            new Row { ["parent_id"] = other["id"], ["label"] = "x" });
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
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var parent = new Row { ["type_id"] = org["id"] };
        var other = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent, other);
        await connection.CreateCommand().Execute("delete", [new Parameter("id", other["id"])]);
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["label"] = "b" },
            new Row { ["parent_id"] = other["id"], ["label"] = "x" });
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
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var home = new Row { ["name"] = "home" };
        var gone = new Row { ["name"] = "gone" };
        var childTypesConnection = await Setup(home, gone);
        await childTypesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
        var parent = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent);
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childTypesRepo = new ReadonlyRepository<TypeRow>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["type_id"] = home["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["type_id"] = gone["id"], ["label"] = "b" });
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
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var home = new Row { ["name"] = "home" };
        var gone = new Row { ["name"] = "gone" };
        var childTypesConnection = await Setup(home, gone);
        await childTypesConnection.CreateCommand().Execute("delete", [new Parameter("id", gone["id"])]);
        var parent = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent);
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var childTypesRepo = new ReadonlyRepository<TypeRow>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["type_id"] = home["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["type_id"] = gone["id"], ["label"] = "b" });
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
        var person = new Row { ["type_name"] = "person" };
        var org = new Row { ["type_name"] = "org" };
        var connection = await Setup(person, org);
        IList<object> parents = [];
        var parentsRepo = new ReadonlyRepository<Parent>(connection);
        await new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
        {
            Key = "id",
            Fields = new Dictionary<string, TypeMapField> { ["id"] = "id", ["type_name"] = "type_name" }
        }).Apply(parentsRepo, new("type_name", "id", "id"), parents, [person, org]);
        var items = parents.Cast<Parent>();
        Assert.Equal([Id(person), Id(org)], items.Select(item => item.id));
        Assert.Equal(["person", "org"], items.Select(item => item.type_name));
    }

    [Fact]
    public async Task LookupKeyComesFromRenamedRow()
    {
        var admin = new Row { ["name"] = "admin" };
        var typesConnection = await Setup(admin);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var usersConnection = await Setup(new Row { ["user_type_id"] = admin["id"] });
        var usersRepo = new ReadonlyRepository<View>(usersConnection, new ReadonlyRepositoryOptions<View>
        {
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
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
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        foreach (var row in rows)
        {
            var response = await connection.CreateCommand().Execute(
                "insert",
                [.. row.Select(pair => new Parameter(pair.Key, pair.Value))]);
            row["id"] = response.Result;
        }
        return connection;
    }

    static int Id(Row row) => (int)((Value.Int)row["id"]).Item;
}
