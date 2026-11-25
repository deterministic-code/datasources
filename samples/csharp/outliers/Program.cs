using Backend.Types.Datasource;
using Backend.Types.View;
using Datasources;
using Outliers;

var lines = await Run();
foreach (var line in lines) Console.WriteLine(line);

public static partial class Program
{
    public static async Task<string[]> Run()
    {
        await using var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var trees = new CategoryTreesRepository(connection);
        var regionTrees = new RegionCategoryTreesRepository(connection);
        var graphs = new NodeGraphsRepository(connection);
        var wards = new WardViewsRepository(connection);
        await trees.Create(new CategoryTree
        {
            Name = "food",
            Children = [new CategoryTree { Name = "fruit", Children = [new CategoryTree { Name = "apple" }] }],
        });
        await regionTrees.Create(new RegionCategoryTree
        {
            RegionId = 1,
            CategoryId = 1,
            Name = "west",
            Children = [new RegionCategoryTree { RegionId = 1, CategoryId = 2, Name = "coast" }],
        });
        await graphs.Create(new NodeGraph
        {
            Value = "core",
            Edges =
            [
                new NodeGraph { Value = "blade", Edges = [new NodeGraph { Value = "core" }] },
                new NodeGraph { Value = "potion" },
            ],
        });
        await wards.Create(new WardView
        {
            RegionId = 1,
            WardId = 1,
            Name = "north",
            Aliases = [new WardAlias { Name = "uptown" }],
            Settings = { ["color"] = "blue" },
            Nodes = [new WardNode { Name = "dock" }],
        });
        await wards.Create(new WardView
        {
            RegionId = 1,
            WardId = 2,
            Name = "south",
            Aliases = [new WardAlias { Name = "midtown" }],
            Settings = { ["color"] = "red" },
            Nodes = [new WardNode { Name = "pier" }],
        });
        var created = await List(wards);
        var north = created.Single(row => row.WardId == 1);
        var south = created.Single(row => row.WardId == 2);
        var createNorth = $"ward create {north.RegionId}-{north.WardId}/{north.Name} alias {north.Aliases[0].ParentRegionId}-{north.Aliases[0].ParentWardId}/{north.Aliases[0].Name} node {north.Nodes[0].ParentRegionId}-{north.Nodes[0].ParentWardId}/{north.Nodes[0].Name} setting {north.Settings["color"]}";
        var createSouth = $"ward create {south.RegionId}-{south.WardId}/{south.Name} alias {south.Aliases[0].ParentRegionId}-{south.Aliases[0].ParentWardId}/{south.Aliases[0].Name} node {south.Nodes[0].ParentRegionId}-{south.Nodes[0].ParentWardId}/{south.Nodes[0].Name} setting {south.Settings["color"]}";
        north.Aliases = [new WardAlias { Name = "downtown" }];
        north.Settings["color"] = "green";
        north.Nodes = [new WardNode { Name = "quay" }];
        await wards.Update(north);
        var updated = (await List(wards)).Single(row => row.WardId == 1);
        var maps = await Mapping.Run(connection);
        return Format.Transcript(
            await List(trees),
            await List(regionTrees),
            await List(graphs),
            maps.Concat(
            [
                createNorth,
                createSouth,
                $"ward update {updated.RegionId}-{updated.WardId}/{updated.Name} alias {updated.Aliases[0].ParentRegionId}-{updated.Aliases[0].ParentWardId}/{updated.Aliases[0].Name} node {updated.Nodes[0].ParentRegionId}-{updated.Nodes[0].ParentWardId}/{updated.Nodes[0].Name} setting {updated.Settings["color"]}",
            ]));
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo)
    {
        var rows = new List<T>();
        await foreach (var row in repo.Read()) rows.Add(row);
        return rows;
    }
}
