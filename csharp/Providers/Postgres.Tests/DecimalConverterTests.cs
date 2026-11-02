using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class DecimalConverterTests
{
    [Fact]
    public void Decimal()
    {
        var converter = new DecimalConverter();
        Assert.Equal(new Value.Text("7"), converter.Convert(7L));
        Assert.Equal(new Value.Text("10.5"), converter.Convert(10.5d));
        Assert.Equal(new Value.Text("10.5"), converter.Convert("10.5"));
        Assert.Equal(new Value.Text("10.5"), converter.Convert("10.50"));
        Assert.Equal(new Value.Text("0"), converter.Convert(0L));
        Assert.Equal(new Value.Text("0"), converter.Convert("0.00"));
        Assert.Equal(new Value.Text("10.5"), converter.ReverseConvert(10.5m));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Bool(true), converter.Convert(true));
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(true));
    }
}
