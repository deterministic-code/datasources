using Datasources;

namespace Datasources.Tests;

public class ToLowerCaseConverterTests
{
    [Fact]
    public void Lowers()
    {
        Assert.Equal("ada", new ToLowerCaseConverter().Convert("ADA"));
        Assert.Equal("ADA", new ToLowerCaseConverter().ReverseConvert("ADA"));
    }
}
