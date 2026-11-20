using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class TreeGraphCreateEnrichmentStrategyTests
{
    sealed class Category
    {
        public long Id { get; set; }
        public long? ParentCategoryId { get; set; }
        public string Name { get; set; } = "";
        public List<Category> Children { get; set; } = [];
    }

    sealed class RegionCategory
    {
        public long RegionId { get; set; }
        public long CategoryId { get; set; }
        public long? ParentRegionId { get; set; }
        public long? ParentCategoryId { get; set; }
        public string Name { get; set; } = "";
        public List<RegionCategory> Children { get; set; } = [];
    }

    sealed class Node
    {
        public long Id { get; set; }
        public string Value { get; set; } = "";
        public List<Node> Edges { get; set; } = [];
    }

    sealed class NodeEdge
    {
        public long FromNodeId { get; set; }
        public long ToNodeId { get; set; }
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    [Fact]
    public async Task TreeCreateWritesParentsFirstByName()
    {
        var connection = await OpenConnection();
        var via = Open<Category>(connection);
        var repo = Open<Category>(connection, new DefaultWriteEnrichmentStrategy<Category>(create: new TreeCreateEnrichmentStrategy<Category>()));
        repo.AddEnriched(new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List), via, new TreeReadonlyEnrichmentStrategy<Category>());
        await repo.Create(new Category
        {
            Name = "food",
            Children = [new Category { Name = "fruit", Children = [new Category { Name = "apple" }] }],
        });
        var food = await repo.ReadSingle(new Where(new Operator.Equals("Name", "food")));
        Assert.Equal("fruit", food!.Children.Single().Name);
        Assert.Equal("apple", food.Children.Single().Children.Single().Name);
        Assert.Empty(food.Children.Single().Children.Single().Children);
    }

    [Fact]
    public async Task TreeCreateThrowsOnACycle()
    {
        var connection = await OpenConnection();
        var via = Open<Category>(connection);
        var repo = Open<Category>(connection, new DefaultWriteEnrichmentStrategy<Category>(create: new TreeCreateEnrichmentStrategy<Category>()));
        repo.AddEnriched(new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List), via);
        var food = new Category { Name = "food" };
        var apple = new Category { Name = "apple", Children = [food] };
        food.Children = [new Category { Name = "fruit", Children = [apple] }];
        var error = await Assert.ThrowsAsync<DatasourceException>(() => repo.Create(food));
        Assert.Contains("food", error.Message);
    }

    [Fact]
    public async Task TreeCreateStampsCompositeParents()
    {
        var connection = await OpenConnection();
        var via = Open<RegionCategory>(connection, null, ["RegionId", "CategoryId"]);
        var repo = Open<RegionCategory>(connection, new DefaultWriteEnrichmentStrategy<RegionCategory>(create: new TreeCreateEnrichmentStrategy<RegionCategory>()), ["RegionId", "CategoryId"]);
        repo.AddEnriched(
            new("Children", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [
                new EnrichmentMatch("RegionId", "ParentRegionId"),
                new EnrichmentMatch("CategoryId", "ParentCategoryId"),
            ]),
            via,
            new TreeReadonlyEnrichmentStrategy<RegionCategory>());
        await repo.Create(new RegionCategory
        {
            RegionId = 1,
            CategoryId = 1,
            Name = "west",
            Children = [new RegionCategory { RegionId = 1, CategoryId = 2, Name = "coast" }],
        });
        var west = await repo.ReadSingle(new Where(new Operator.Equals("Name", "west")));
        Assert.Equal("coast", west!.Children.Single().Name);
        Assert.Equal(1, west.Children.Single().ParentRegionId);
        Assert.Equal(1, west.Children.Single().ParentCategoryId);
    }

    [Fact]
    public async Task GraphCreateWritesNodesThenEdgesAndSkipsADiamond()
    {
        var nodes = await OpenConnection();
        var edges = await OpenConnection();
        var via = Open<Node>(nodes);
        var link = Open<NodeEdge>(edges);
        var repo = Open<Node>(nodes, new DefaultWriteEnrichmentStrategy<Node>(create: new GraphCreateEnrichmentStrategy<Node, NodeEdge>(link)));
        repo.AddEnriched(
            new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")),
            via,
            new GraphReadonlyEnrichmentStrategy<Node, NodeEdge>(link));
        await repo.Create(new Node
        {
            Value = "a",
            Edges =
            [
                new Node { Value = "b", Edges = [new Node { Value = "d" }] },
                new Node { Value = "c", Edges = [new Node { Value = "d" }] },
            ],
        });
        Assert.Equal(4, (await List(via)).Count);
        Assert.Equal(4, (await List(link)).Count);
        var loaded = await repo.ReadSingle(new Where(new Operator.Equals("Value", "a")));
        Assert.Equal(2, loaded!.Edges.Count);
    }

    [Fact]
    public async Task GraphCreateWritesALoop()
    {
        var nodes = await OpenConnection();
        var edges = await OpenConnection();
        var via = Open<Node>(nodes);
        var link = Open<NodeEdge>(edges);
        var repo = Open<Node>(nodes, new DefaultWriteEnrichmentStrategy<Node>(create: new GraphCreateEnrichmentStrategy<Node, NodeEdge>(link)));
        repo.AddEnriched(
            new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")),
            via,
            new GraphReadonlyEnrichmentStrategy<Node, NodeEdge>(link));
        var a = new Node { Value = "a" };
        var b = new Node { Value = "b", Edges = [a] };
        a.Edges = [b];
        await repo.Create(a);
        Assert.Equal(2, (await List(via)).Count);
        Assert.Equal(2, (await List(link)).Count);
        var loadedA = await repo.ReadSingle(new Where(new Operator.Equals("Value", "a")));
        Assert.Equal("b", loadedA!.Edges.Single().Value);
        var loadedB = await repo.ReadSingle(new Where(new Operator.Equals("Value", "b")));
        Assert.Equal("a", loadedB!.Edges.Single().Value);
    }

    static CRUDRepository<T> Open<T>(IConnection connection, IWriteEnrichmentStrategy<T>? write = null, string[]? ids = null) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Ids = ids ?? ["Id"],
            PropertyMappings = Snake,
            Write = write,
        });

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo)
    {
        var rows = new List<T>();
        await foreach (var row in repo.Read()) rows.Add(row);
        return rows;
    }

    static async Task<IConnection> OpenConnection()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }
}
