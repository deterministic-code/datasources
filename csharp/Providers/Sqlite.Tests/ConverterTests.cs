using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class ConverterTests
{
    [Fact]
    public void RegistersSqliteTypes()
    {
        _ = SqliteTypes.All;
        Assert.True(DefaultTypeConverters.For("sqlite").ContainsKey("bool"));
    }

    [Fact]
    public void BoolIsInteger()
    {
        var converter = new BoolConverter();
        Assert.Equal(new Value.Bool(true), converter.Convert(1L));
        Assert.Equal(new Value.Bool(false), converter.Convert(0L));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(true));
        Assert.Equal(new Value.Int(0), converter.ReverseConvert(false));
    }

    [Fact]
    public void DateTimeUuidDecimalRoundTrip()
    {
        Assert.IsType<Value.DateTime>(new DateTimeConverter().Convert("2020-01-02T03:04:05.000Z"));
        Assert.IsType<Value.Guid>(new UuidConverter().Convert("550e8400-e29b-41d4-a716-446655440000"));
        Assert.Equal(new Value.Decimal(10.5m), new DecimalConverter().Convert("10.5"));
        var bytes = new Value.Binary([1, 2]);
        Assert.Same(bytes, new BinaryConverter().Convert(bytes));
    }
}
