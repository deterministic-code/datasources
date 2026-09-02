using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class BinaryConverterTests
{
    [Fact]
    public void Binary()
    {
        var converter = new BinaryConverter();
        var bytes = new Value.Binary([1, 2, 3, 4]);
        Assert.Same(bytes, converter.Convert(bytes));
        Assert.Same(bytes, converter.ReverseConvert(bytes));
        Assert.Equal(bytes, converter.ReverseConvert("AQIDBA=="));
    }
}
