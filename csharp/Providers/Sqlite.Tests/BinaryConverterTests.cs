using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class BinaryConverterTests
{
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
