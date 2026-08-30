using Datasources;

namespace Datasources.Tests;

public class ConverterTests
{
    public class User
    {
        public int Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
        public DateTime Created { get; set; }
    }

    public class Mixed
    {
        public string Label { get; set; } = "";
        public long Count { get; set; }
        public int this[int i] { get => 0; set { } }
    }

    public class WriteOnly
    {
        public int Id { get; set; }
        public int Secret { set { } }
    }

    public class Extra
    {
        public Guid Id { get; set; }
        public byte[] Blob { get; set; } = [];
        public decimal Amount { get; set; }
        public int? Count { get; set; }
    }

    [Fact]
    public void ConvertAndReverseConvertRoundTrip()
    {
        var converter = new TypeConverter<User>();
        var user = converter.Convert(new Row
        {
            ["Id"] = 7,
            ["Username"] = "ada",
            ["IsEnabled"] = 1,
            ["Created"] = "2020-01-02T03:04:05.000Z"
        });
        Assert.Equal(7, user.Id);
        Assert.Equal("ada", user.Username);
        Assert.True(user.IsEnabled);
        Assert.Equal(2020, user.Created.Year);
        var row = converter.ReverseConvert(user);
        Assert.Equal(new Value.Int(7), row["Id"]);
        Assert.Equal(new Value.Text("ada"), row["Username"]);
    }

    [Fact]
    public void MapsDictionaryRows()
    {
        var converter = new TypeConverter<Dictionary<string, Value>>("items");
        var dest = converter.Convert(new Row { ["display_name"] = "a" });
        Assert.Equal(new Value.Text("a"), dest["display_name"]);
        Assert.Equal(new Value.Text("a"), converter.ReverseConvert(dest)["display_name"]);
    }

    [Fact]
    public void SkipsIndexersAndWriteOnlyProperties()
    {
        var mixed = new TypeConverter<Mixed>().Convert(new Row { ["Label"] = "x", ["Count"] = 1L, ["Item"] = 2 });
        Assert.Equal("x", mixed.Label);
        Assert.Equal(1, mixed.Count);
        var obj = new TypeConverter<WriteOnly>().Convert(new Row { ["Id"] = 1, ["missing"] = 2, ["Secret"] = 3 });
        Assert.Equal(1, obj.Id);
        var extra = new TypeConverter<Extra>().Convert(new Row
        {
            ["Id"] = "550e8400-e29b-41d4-a716-446655440000",
            ["Blob"] = "aGk=",
            ["Amount"] = "1.25",
            ["Count"] = Value.Null.Instance
        });
        Assert.Equal(Guid.Parse("550e8400-e29b-41d4-a716-446655440000"), extra.Id);
        Assert.Equal("hi"u8.ToArray(), extra.Blob);
        Assert.Equal(1.25m, extra.Amount);
        Assert.Null(extra.Count);
    }
}
