using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class TreeGraphReadonlyEnrichmentStrategyTests
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
    public async Task TreeFillsThreeLevelsAndStopsOnACycle()
    {
        var foodRow = new Row { ["name"] = "food" };
        var fruitRow = new Row { ["name"] = "fruit" };
        var appleRow = new Row { ["name"] = "apple" };
        var connection = await Setup(foodRow, fruitRow, appleRow);
        var foodId = Id(foodRow);
        var fruitId = Id(fruitRow);
        var appleId = Id(appleRow);
        await SetParent(connection, fruitId, foodId);
        await SetParent(connection, appleId, fruitId);
        await SetParent(connection, foodId, appleId);
        var via = Open<Category>(connection);
        var repo = Open<Category>(connection);
        repo.AddEnriched(
            new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List),
            via,
            new TreeReadonlyEnrichmentStrategy<Category>());
        var food = await repo.ReadSingle(new Where(new Operator.Equals("Id", foodId)));
        Assert.Equal("food", food!.Name);
        Assert.Equal("fruit", food.Children.Single().Name);
        Assert.Equal("apple", food.Children.Single().Children.Single().Name);
        Assert.Empty(food.Children.Single().Children.Single().Children);
    }

    [Fact]
    public async Task TreeMatchesACompositeParent()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        await connection.CreateCommand().Execute("insert", [
            new Parameter("region_id", 1),
            new Parameter("category_id", 10),
            new Parameter("name", "food"),
        ]);
        await connection.CreateCommand().Execute("insert", [
            new Parameter("region_id", 1),
            new Parameter("category_id", 11),
            new Parameter("parent_region_id", 1),
            new Parameter("parent_category_id", 10),
            new Parameter("name", "fruit"),
        ]);
        var via = new ReadonlyRepository<RegionCategory>(connection, new ReadonlyRepositoryMappingOptions<RegionCategory>
        {
            PropertyMappings = Snake,
        });
        var repo = new ReadonlyRepository<RegionCategory>(connection, new ReadonlyRepositoryMappingOptions<RegionCategory>
        {
            PropertyMappings = Snake,
        });
        repo.AddEnriched(
            new("Children", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [
                new EnrichmentMatch("RegionId", "ParentRegionId"),
                new EnrichmentMatch("CategoryId", "ParentCategoryId"),
            ]),
            via,
            new TreeReadonlyEnrichmentStrategy<RegionCategory>());
        var food = await repo.ReadSingle(new Where(new Operator.Equals("CategoryId", 10)));
        Assert.Equal("fruit", food!.Children.Single().Name);
        Assert.Empty(food.Children.Single().Children);
    }

    [Fact]
    public async Task GraphFillsEdgesStopsACycleAndCopiesADiamond()
    {
        var aRow = new Row { ["value"] = "a" };
        var bRow = new Row { ["value"] = "b" };
        var cRow = new Row { ["value"] = "c" };
        var dRow = new Row { ["value"] = "d" };
        var nodes = await Setup(aRow, bRow, cRow, dRow);
        var ids = new[] { Id(aRow), Id(bRow), Id(cRow), Id(dRow) };
        var edges = new MemoryDriver().Connection(null);
        await edges.Open();
        await InsertEdge(edges, ids[0], ids[1]);
        await InsertEdge(edges, ids[0], ids[2]);
        await InsertEdge(edges, ids[1], ids[3]);
        await InsertEdge(edges, ids[2], ids[3]);
        await InsertEdge(edges, ids[3], ids[0]);
        var via = Open<Node>(nodes);
        var link = new ReadonlyRepository<NodeEdge>(edges, new ReadonlyRepositoryMappingOptions<NodeEdge>
        {
            PropertyMappings = Snake,
        });
        var repo = Open<Node>(nodes);
        repo.AddEnriched(
            new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")),
            via,
            new GraphReadonlyEnrichmentStrategy<Node, NodeEdge>(link));
        var a = await repo.ReadSingle(new Where(new Operator.Equals("Id", ids[0])));
        Assert.Equal(2, a!.Edges.Count);
        var b = a.Edges.Single(node => node.Value == "b");
        var c = a.Edges.Single(node => node.Value == "c");
        Assert.Equal("d", b.Edges.Single().Value);
        Assert.Equal("d", c.Edges.Single().Value);
        Assert.True(b.Edges.Single().Edges.Count == 0 || c.Edges.Single().Edges.Count == 0);
        var loopRow = new Row { ["value"] = "loop" };
        var self = await Setup(loopRow);
        var loopId = Id(loopRow);
        var loopEdges = new MemoryDriver().Connection(null);
        await loopEdges.Open();
        await InsertEdge(loopEdges, loopId, loopId);
        var loopVia = Open<Node>(self);
        var loopLink = new ReadonlyRepository<NodeEdge>(loopEdges, new ReadonlyRepositoryMappingOptions<NodeEdge>
        {
            PropertyMappings = Snake,
        });
        var loopRepo = Open<Node>(self);
        loopRepo.AddEnriched(
            new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")),
            loopVia,
            new GraphReadonlyEnrichmentStrategy<Node, NodeEdge>(loopLink));
        var loop = await loopRepo.ReadSingle();
        Assert.Empty(loop!.Edges);
    }

    static ReadonlyRepository<T> Open<T>(IConnection connection) where T : new() =>
        new(connection, new ReadonlyRepositoryMappingOptions<T> { PropertyMappings = Snake });

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

    static Task SetParent(IConnection connection, long id, long parent) =>
        connection.CreateCommand().Execute("update", [new Parameter("id", id), new Parameter("parent_category_id", parent)]);

    static Task InsertEdge(IConnection connection, long from, long to) =>
        connection.CreateCommand().Execute("insert", [new Parameter("from_node_id", from), new Parameter("to_node_id", to)]);
}
