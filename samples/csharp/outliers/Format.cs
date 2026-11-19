using Backend.Types.View;

namespace Outliers;

public static class Format
{
    public static string TreeLine(CategoryTree node)
    {
        var kids = node.Children.OrderBy(child => child.Name).Select(TreeLine);
        return string.Join(" ", new[] { node.Name }.Concat(kids));
    }

    public static string RegionLine(RegionCategoryTree node)
    {
        var kids = node.Children.OrderBy(child => child.Name).Select(RegionLine);
        return string.Join(" ", new[] { $"{node.RegionId}-{node.CategoryId}/{node.Name}" }.Concat(kids));
    }

    public static string GraphLine(NodeGraph node)
    {
        var edges = node.Edges.Select(edge => edge.Value).OrderBy(value => value).ToArray();
        return edges.Length == 0 ? node.Value : $"{node.Value} {string.Join(",", edges)}";
    }

    public static string[] Transcript(IEnumerable<CategoryTree> trees, IEnumerable<RegionCategoryTree> regions, IEnumerable<NodeGraph> graphs) =>
        trees.Select(row => $"tree {TreeLine(row)}")
            .Concat(regions.Select(row => $"region {RegionLine(row)}"))
            .Concat(graphs.Select(row => $"graph {GraphLine(row)}"))
            .OrderBy(line => line)
            .ToArray();
}
