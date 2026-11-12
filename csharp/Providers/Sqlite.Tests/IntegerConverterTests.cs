using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class IntegerConverterTests
{
    [Fact]
    public void Integer()
    {
        var converter = new IntegerConverter();
        Assert.Equal(new Value.Int(10), converter.Convert("10"));
        Assert.Equal(new Value.Int(0), converter.Convert("0"));
        Assert.Throws<FormatException>(() => converter.Convert(""));
        Assert.Throws<FormatException>(() => converter.Convert("1.0"));
        Assert.Equal(new Value.Int(7), converter.Convert(7L));
        Assert.Equal(new Value.Int(7), converter.ReverseConvert(7L));
    }
}
