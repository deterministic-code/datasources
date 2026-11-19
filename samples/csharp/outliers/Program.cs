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
        var regions = new RegionCategoriesRepository(connection);
        var nodes = new NodesRepository(connection);
        var edges = new NodeEdgesRepository(connection);
        var trees = new CategoryTreesRepository(connection);
        var regionTrees = new RegionCategoryTreesRepository(connection);
        var graphs = new NodeGraphsRepository(connection);
        await regions.Create(new RegionCategory { RegionId = 1, CategoryId = 1, Name = "west" });
        await regions.Create(new RegionCategory { RegionId = 1, CategoryId = 2, ParentRegionId = 1, ParentCategoryId = 1, Name = "coast" });
        var core = await nodes.Create(new Node { Value = "core" });
        var blade = await nodes.Create(new Node { Value = "blade" });
        var potion = await nodes.Create(new Node { Value = "potion" });
        await edges.Create(new NodeEdge { FromNodeId = core.Id, ToNodeId = blade.Id });
        await edges.Create(new NodeEdge { FromNodeId = core.Id, ToNodeId = potion.Id });
        await edges.Create(new NodeEdge { FromNodeId = blade.Id, ToNodeId = core.Id });
        return Format.Transcript(await List(trees), await List(regionTrees), await List(graphs));
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo)
    {
        var rows = new List<T>();
        await foreach (var row in repo.Read()) rows.Add(row);
        return rows;
    }
}
