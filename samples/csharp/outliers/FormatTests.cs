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
            ["graph core blade", "map create text_to_integer 10", "region 1-1/west 1-2/coast", "tree food fruit apple"],
            Format.Transcript(
                [food],
                [root],
                [new NodeGraph { Value = "core", Edges = [new() { Value = "blade" }] }],
                ["map create text_to_integer 10"]));
        var token = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        var opened = new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        var row = new MappingCell
        {
            Id = 1,
            TextToString = "ada",
            TextToInteger = 10,
            TextToDecimal = 10.5m,
            TextToDatetime = opened,
            TextToBinary = [1, 2],
            TextToGuid = token,
            IntToInteger = 10,
            IntToFloat = 10.5,
            IntToDecimal = 7,
            IntToBoolean = true,
            FloatToInteger = 10,
            FloatToFloat = 10.9,
            FloatToDecimal = 10.5m,
            DecimalToInteger = 10,
            DecimalToFloat = 1.2,
            DecimalToDecimal = 10.5m,
            BoolToBoolean = true,
            DatetimeToDatetime = opened,
            BinaryToBinary = [1, 2],
            BinaryToGuid = token,
            GuidToString = token.ToString("D"),
            GuidToGuid = token,
        };
        Assert.Contains("map create text_to_guid 550e8400-e29b-41d4-a716-446655440000", Format.MapLines("create", row));
        Assert.Contains("map create text_to_binary 0102", Format.MapLines("create", row));
        Assert.Contains("map create int_to_boolean true", Format.MapLines("create", row));
        Assert.Contains("map create id 1", Format.MapLines("create", row));
        Assert.Equal(["map read error \"10.9\" is not a long"], Format.MapError("read", "\"10.9\" is not a long"));
        Assert.Contains("map create guid_to_guid", Format.MapLines("create", new MappingCell()));
        Assert.Contains("map create text_to_string", Format.MapLines("create", new MappingCell()));
        Assert.Equal(
            ["graph core blade", "region 1-1/west 1-2/coast", "tree food fruit apple"],
            Format.Transcript([food], [root], [new NodeGraph { Value = "core", Edges = [new() { Value = "blade" }] }]));
    }
}
