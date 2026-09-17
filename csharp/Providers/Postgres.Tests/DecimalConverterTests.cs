using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class DecimalConverterTests
{
    [Fact]
    public void Decimal()
    {
        var converter = new DecimalConverter();
        Assert.Equal(new Value.Decimal(7m), converter.Convert(7L));
        Assert.Equal(new Value.Decimal(10.5m), converter.Convert("10.5"));
        Assert.Equal(new Value.Decimal(10.5m), converter.ReverseConvert(10.5m));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Bool(true), converter.Convert(true));
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(true));
    }
}
