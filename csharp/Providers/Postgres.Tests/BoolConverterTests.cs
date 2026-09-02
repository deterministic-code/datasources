using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class BoolConverterTests
{
    [Fact]
    public void Bool()
    {
        var converter = new BoolConverter();
        Assert.Equal(new Value.Bool(true), converter.Convert(1L));
        Assert.Equal(new Value.Bool(false), converter.Convert(0L));
        Assert.Equal(new Value.Text("x"), converter.Convert("x"));
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(true));
        Assert.Equal(new Value.Text("x"), converter.ReverseConvert("x"));
    }
}
