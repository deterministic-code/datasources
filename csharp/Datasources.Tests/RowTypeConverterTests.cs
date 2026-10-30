using Datasources;

namespace Datasources.Tests;

public class RowTypeConverterTests
{
    public class User
    {
        public long Id { get; set; }
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

    public class WithAttributes
    {
        public long Id { get; set; }
        public Dictionary<string, string> Attributes { get; set; } = [];
    }

    public class WriteOnly
    {
        public long Id { get; set; }
        public int Secret { set { } }
    }

    [Fact]
    public void ConvertAndReverseConvertRoundTrip()
    {
        var created = new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        var converter = new MetaDataRowTypeConverter<User>();
        var user = converter.Convert(new Row
        {
            ["Id"] = 7L,
            ["Username"] = "ada",
            ["IsEnabled"] = true,
            ["Created"] = created
        });
        Assert.Equal(7L, user.Id);
        Assert.Equal("ada", user.Username);
        Assert.True(user.IsEnabled);
        Assert.Equal(created, user.Created);
        var row = converter.ReverseConvert(user);
        Assert.Equal(new Value.Int(7), row["Id"]);
        Assert.Equal(new Value.Text("ada"), row["Username"]);
    }

    [Fact]
    public void MapsDictionaryRows()
    {
        var converter = new MetaDataRowTypeConverter<Dictionary<string, Value>>();
        var dest = converter.Convert(new Row { ["display_name"] = "a" });
        Assert.Equal(new Value.Text("a"), dest["display_name"]);
        Assert.Equal(new Value.Text("a"), converter.ReverseConvert(dest)["display_name"]);
    }

    sealed class Counted
    {
        public int id { get; set; }
    }

    [Fact]
    public void LeavesIntPropertyWhenCellIsLong()
    {
        var row = new MetaDataRowTypeConverter<Counted>().Convert(new Row { ["id"] = 1L });
        Assert.Equal(0, row.id);
    }

    [Fact]
    public void CoercesIntPropertyWhenShouldCoerceId()
    {
        var converter = new MetaDataRowTypeConverter<Counted> { ShouldCoerceId = true };
        Assert.Equal(1, converter.Convert(new Row { ["id"] = 1L }).id);
        Assert.Throws<OverflowException>(() => converter.Convert(new Row { ["id"] = (long)int.MaxValue + 1 }));
    }

    [Fact]
    public void CoercesGuidOntoString()
    {
        var id = System.Guid.Parse("54ef41f2-0cb4-4c72-97a7-776fd183f5ed");
        var row = new MetaDataRowTypeConverter<Keyed>().Convert(new Row { ["Uuid"] = new Value.Guid(id) });
        Assert.Equal(id.ToString(), row.Uuid);
    }

    public class Keyed
    {
        public string Uuid { get; set; } = "";
    }

    [Fact]
    public void SkipsIndexersAndWriteOnlyProperties()
    {
        var mixed = new MetaDataRowTypeConverter<Mixed>().Convert(new Row { ["Label"] = "x", ["Count"] = 1L, ["Item"] = 2 });
        Assert.Equal("x", mixed.Label);
        Assert.Equal(1, mixed.Count);
        var obj = new MetaDataRowTypeConverter<WriteOnly>().Convert(new Row { ["Id"] = 1, ["missing"] = 2, ["Secret"] = 3 });
        Assert.Equal(1L, obj.Id);
    }

    sealed class OnlyOne : TypeConverter<Value, Value>
    {
        public override Value Convert(Value from) => from.ToObject() is long n && n == 1;
        public override Value ReverseConvert(Value to) => to;
    }

    [Fact]
    public void EmptyTextLeavesDictionaryAtDefault()
    {
        var item = new MetaDataRowTypeConverter<WithAttributes>().Convert(new Row
        {
            ["Id"] = 1L,
            ["Attributes"] = ""
        });
        Assert.Equal(1L, item.Id);
        Assert.Empty(item.Attributes);
    }

    [Fact]
    public void EmptyTextLeavesIntegerAtDefault()
    {
        var user = new MetaDataRowTypeConverter<User>().Convert(new Row
        {
            ["Id"] = "",
            ["Username"] = "ada"
        });
        Assert.Equal(0L, user.Id);
        Assert.Equal("ada", user.Username);
    }

    [Fact]
    public void FieldOverrideBeatsTypeDefault()
    {
        var row = new Row
        {
            ["Id"] = 1,
            ["Username"] = "a",
            ["IsEnabled"] = 2,
            ["Created"] = new DateTime(2020, 1, 2, 0, 0, 0, DateTimeKind.Utc)
        };
        var converter = new MetaDataRowTypeConverter<User>(new Dictionary<string, TypeConverter<Value, Value>>
        {
            ["IsEnabled"] = new OnlyOne()
        });
        Assert.False(converter.Convert(row).IsEnabled);
    }

    [Fact]
    public void FromUsesIntegerConverterForIntegerFamily()
    {
        var converter = RowTypeConverter.From(
            new Dictionary<string, TypeConverter<Value, Value>> { ["integer"] = new OnlyOne() },
            new Dictionary<string, string> { ["age"] = "unsignedinteger", ["name"] = "string" });
        Assert.Equal(new Value.Bool(true), converter.Convert(new Row { ["age"] = 1L, ["name"] = "a" })["age"]);
        Assert.Equal(new Value.Text("a"), converter.Convert(new Row { ["age"] = 1L, ["name"] = "a" })["name"]);
    }

    [Fact]
    public void AppliesFieldOverrideOnDictionaryRows()
    {
        var converter = new MetaDataRowTypeConverter<Dictionary<string, Value>>(new Dictionary<string, TypeConverter<Value, Value>>
        {
            ["flag"] = new OnlyOne()
        });
        var dest = converter.Convert(new Row { ["flag"] = 1L, ["name"] = "a" });
        Assert.Equal(new Value.Bool(true), dest["flag"]);
        Assert.Equal(new Value.Text("a"), dest["name"]);
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(dest)["flag"]);
    }
}
