using Backend.Types.View;
using Outliers;

public sealed class FormatTests
{
    [Fact]
    public void TranscriptCoversTreeRegionAndGraph()
    {
        var apple = new CategoryTree { Name = "apple" };
        var fruit = new CategoryTree { Name = "fruit", Children = [apple] };
        var food = new CategoryTree { Name = "food", Children = [fruit] };
        Assert.Equal("food fruit apple", Format.TreeLine(food));
        Assert.Equal("apple", Format.TreeLine(apple));
        var child = new RegionCategoryTree { RegionId = 1, CategoryId = 2, Name = "coast" };
        var root = new RegionCategoryTree { RegionId = 1, CategoryId = 1, Name = "west", Children = [child] };
        Assert.Equal("1-1/west 1-2/coast", Format.RegionLine(root));
        Assert.Equal("core blade,potion", Format.GraphLine(new NodeGraph { Value = "core", Edges = [new() { Value = "potion" }, new() { Value = "blade" }] }));
        Assert.Equal("blade", Format.GraphLine(new NodeGraph { Value = "blade" }));
        Assert.Equal(
            ["graph core blade", "region 1-1/west 1-2/coast", "tree food fruit apple"],
            Format.Transcript([food], [root], [new NodeGraph { Value = "core", Edges = [new() { Value = "blade" }] }]));
    }
}
