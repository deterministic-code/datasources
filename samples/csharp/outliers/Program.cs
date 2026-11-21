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
        var maps = await Mapping.Run(connection);
        return Format.Transcript(await List(trees), await List(regionTrees), await List(graphs), maps);
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo)
    {
        var rows = new List<T>();
        await foreach (var row in repo.Read()) rows.Add(row);
        return rows;
    }
}
