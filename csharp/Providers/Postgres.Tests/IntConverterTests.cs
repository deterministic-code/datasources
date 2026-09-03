using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class IntConverterTests
{
    [Fact]
    public void Int()
    {
        var converter = new IntConverter();
        Assert.Equal(new Value.Int(7), converter.Convert("7"));
        Assert.Equal(new Value.Int(7), converter.ReverseConvert("7"));
        Assert.Equal(new Value.Int(7), converter.ReverseConvert(7L));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
    }
}
