using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class ReadonlyRepositoryTests
{
    sealed class UserType
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Parent
    {
        public long Id { get; set; }
        public UserType Type { get; set; } = new();
        public string TypeName { get; set; } = "";
        public List<Child> Children { get; set; } = [];
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public UserType Type { get; set; } = new();
        public string TypeName { get; set; } = "";
        public string Label { get; set; } = "";
    }

    sealed class View
    {
        public string UserType { get; set; } = "";
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly DictionaryMapper<string> References = new(new Dictionary<string, string>
    {
        ["Type"] = "type_id",
    });

    static readonly IMapper<string, string> Columns = new TransformMapper<string, string>(
        name => References.Map(name) is var mapped && mapped != name ? mapped : Snake.Map(name),
        name => References.ReverseMap(name) is var from && from != name ? from : Snake.ReverseMap(name));

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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("person", items[0].TypeName);
        Assert.Equal("", items[1].TypeName);
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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        parentsRepo.AddEnriched(
            new("TypeName", "Type", "Id", "Name"),
            typesRepo,
            new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>());
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("person", items[0].TypeName);
        Assert.Equal("", items[1].TypeName);
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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["label"] = "b" },
            new Row { ["parent_id"] = other["id"], ["label"] = "x" });
        var childrenRepo = Open<Child>(childrenConnection);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.TypeName);
        Assert.Equal(["a", "b"], item.Children.Select(child => child.Label));
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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["label"] = "b" },
            new Row { ["parent_id"] = other["id"], ["label"] = "x" });
        var childrenRepo = Open<Child>(childrenConnection);
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo, strategy);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo, strategy);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.TypeName);
        Assert.Equal(["a", "b"], item.Children.Select(child => child.Label));
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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        var childTypesRepo = Open<UserType>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["type_id"] = home["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["type_id"] = gone["id"], ["label"] = "b" });
        var childrenRepo = Open<Child>(childrenConnection);
        childrenRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), childTypesRepo);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.TypeName);
        Assert.Equal("home", item.Children[0].TypeName);
        Assert.Equal("", item.Children[1].TypeName);
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
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(typesConnection);
        var childTypesRepo = Open<UserType>(childTypesConnection);
        var childrenConnection = await Setup(
            new Row { ["parent_id"] = parent["id"], ["type_id"] = home["id"], ["label"] = "a" },
            new Row { ["parent_id"] = parent["id"], ["type_id"] = gone["id"], ["label"] = "b" });
        var childrenRepo = Open<Child>(childrenConnection);
        var strategy = new HashsetOptimizedReadonlyEnrichmentStrategy<Parent>();
        childrenRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), childTypesRepo);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo, strategy);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo, strategy);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.TypeName);
        Assert.Equal("home", item.Children[0].TypeName);
        Assert.Equal("", item.Children[1].TypeName);
    }

    [Fact]
    public async Task FlatToNestedCollapsesJoinRows()
    {
        var person = new Row { ["TypeName"] = "person" };
        var org = new Row { ["TypeName"] = "org" };
        var connection = await Setup(person, org);
        person["Id"] = person["id"];
        org["Id"] = org["id"];
        IList<Parent> parents = [];
        var parentsRepo = Open<Parent>(connection);
        await new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
        {
            Key = "Id",
            Fields = new Dictionary<string, TypeMapField> { ["Id"] = "Id", ["TypeName"] = "TypeName" }
        }).Apply(parentsRepo, new("TypeName", "Id", "Id"), parents, [person, org]);
        var items = parents;
        Assert.Equal([Id(person), Id(org)], items.Select(item => item.Id));
        Assert.Equal(["person", "org"], items.Select(item => item.TypeName));
    }

    [Fact]
    public async Task LookupKeyComesFromRenamedRow()
    {
        var admin = new Row { ["name"] = "admin" };
        var typesConnection = await Setup(admin);
        var typesRepo = Open<UserType>(typesConnection);
        var usersConnection = await Setup(new Row { ["user_type_id"] = admin["id"] });
        var usersRepo = new ReadonlyRepository<View>(usersConnection, new ReadonlyRepositoryOptions<View>
        {
            PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["UserType"] = "user_type_id",
            }),
        });
        usersRepo.AddEnriched(new("UserType", "UserType", "Id", "Name"), typesRepo);
        var item = await usersRepo.ReadSingle();
        Assert.Equal("admin", item!.UserType);
    }

    static ReadonlyRepository<T> Open<T>(IConnection connection) where T : new() =>
        new(connection, new ReadonlyRepositoryOptions<T> { PropertyMappings = Columns });

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

    static long Id(Row row) => ((Value.Int)row["id"]).Item;
}
