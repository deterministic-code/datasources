using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class BoolConverterTests
{
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
}
