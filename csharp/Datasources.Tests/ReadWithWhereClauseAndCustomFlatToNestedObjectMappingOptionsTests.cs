using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class ReadWithWhereClauseAndCustomFlatToNestedObjectMappingOptionsTests
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
        public string Label { get; set; } = "";
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly DictionaryMapper<string> References = new(new Dictionary<string, string>
    {
        ["Type"] = "type_id",
    });

    static readonly IMapper<string, string> Columns = new TransformMapper<string, string>(
        name => References.Map(name) is var mapped && mapped != name ? mapped : Snake.Map(name),
        name => References.ReverseMap(name) is var from && from != name ? from : Snake.ReverseMap(name));

    static readonly IMapper<string, string> JoinEntity = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["Parent"] = "parent",
        ["Child"] = "child",
    });

    [Fact]
    public async Task ReadWithWhereClauseAndCustomFlatToNestedObjectMappingOptions()
    {
        var connection = await Setup();
        var typesRepo = TypesRepo(connection);
        var type = await typesRepo.Create(new UserType { Name = "person" });
        var parentsRepo = ParentsRepo(connection);
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        await parentsRepo.Create(new Parent { Type = new UserType { Id = type.Id } });
        var enriched = await parentsRepo.ReadSingle();

        var row1 = new Row { ["Id"] = 1L, ["TypeName"] = "person", ["ChildId"] = 10L, ["Label"] = "a" };
        var row2 = new Row { ["Id"] = 1L, ["TypeName"] = "person", ["ChildId"] = 11L, ["Label"] = "b" };
        var mockConnection = new MockConnection { Provider = "memory" };
        var command = new MockCommand(mockConnection);
        mockConnection.Command = command;
        var joinRepo = ParentsRepo(mockConnection);
        joinRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        joinRepo.AddEnriched(
            new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
            joinRepo,
            new NestedToObjectEnrichmentStrategy<Parent>(new TypeMap
            {
                Key = "Id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["Id"] = "Id",
                    ["TypeName"] = "TypeName",
                    ["Children"] = new TypeMapGroup
                    {
                        Key = "ChildId",
                        Fields = new Dictionary<string, TypeMapField>
                        {
                            ["Id"] = "ChildId",
                            ["Label"] = "Label",
                        },
                    },
                },
            }));
        var join = new SelectStatement(new()
        {
            Table = "Parent",
            Fields =
            [
                "Parent.Id",
                "Parent.TypeName",
                "Child.Id",
                "Child.Label",
            ],
            Joins = [Join.Left("Child", "Parent.Id", "Child.ParentId")],
            Where = new Where(new Operator.Equals("Parent.Id", 1)),
        });
        command.Reader = new MockReader<Row>([row1, row2]);
        var items = await joinRepo.Read(
            join,
            null,
            new ReadonlyRepositoryMappingOptions<Parent> { EntityMapper = JoinEntity }).ToListAsync();
        Assert.Equal("person", enriched!.TypeName);
        var parent = Assert.Single(items);
        Assert.Equal(1L, parent.Id);
        Assert.Equal("person", parent.TypeName);
        Assert.Equal(["a", "b"], parent.Children.Select(child => child.Label));
    }

    static CRUDRepository<UserType> TypesRepo(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<UserType>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
        });

    static CRUDRepository<Parent> ParentsRepo(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Parent>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
        });

    static async Task<IConnection> Setup()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }
}
