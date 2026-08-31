using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class ConverterTests
{
    [Fact]
    public void RegistersSqliteTypes()
    {
        _ = SqliteTypes.All;
        var types = DefaultTypeConverters.For("sqlite");
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
        Assert.Empty(DefaultTypeConverters.For("postgres"));
    }

    [Fact]
    public void Bool()
    {
        var converter = new BoolConverter();
        Assert.Equal(new Value.Bool(true), converter.Convert(1L));
        Assert.Equal(new Value.Bool(false), converter.Convert(0L));
        Assert.Equal(new Value.Text("x"), converter.Convert("x"));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(true));
        Assert.Equal(new Value.Int(0), converter.ReverseConvert(false));
        Assert.Equal(new Value.Text("x"), converter.ReverseConvert("x"));
    }

    [Fact]
    public void DateTime()
    {
        var converter = new DateTimeConverter();
        var native = converter.Convert("2020-01-02T03:04:05.000Z");
        Assert.Equal(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc), ((Value.DateTime)native).Item);
        Assert.Equal(new Value.Text("2020-01-02T03:04:05.000Z"), converter.ReverseConvert(native));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Int(1), converter.Convert(1L));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(1L));
    }

    [Fact]
    public void Uuid()
    {
        var converter = new UuidConverter();
        var id = System.Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        var native = converter.Convert("550e8400-e29b-41d4-a716-446655440000");
        Assert.Equal(new Value.Guid(id), native);
        Assert.Equal(new Value.Text(id.ToString()), converter.ReverseConvert(native));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Int(1), converter.Convert(1L));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(1L));
    }

    [Fact]
    public void Decimal()
    {
        var converter = new DecimalConverter();
        Assert.Equal(new Value.Decimal(7m), converter.Convert(7L));
        Assert.Equal(new Value.Decimal(10.5m), converter.Convert("10.5"));
        Assert.Equal(new Value.Text("10.5"), converter.ReverseConvert(10.5m));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Bool(true), converter.Convert(true));
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(true));
    }

    [Fact]
    public void Binary()
    {
        var converter = new BinaryConverter();
        var bytes = new Value.Binary([1, 2]);
        Assert.Same(bytes, converter.Convert(bytes));
        Assert.Same(bytes, converter.ReverseConvert(bytes));
        Assert.Equal(new Value.Text("x"), converter.Convert("x"));
        Assert.Equal(new Value.Text("x"), converter.ReverseConvert("x"));
    }
}
