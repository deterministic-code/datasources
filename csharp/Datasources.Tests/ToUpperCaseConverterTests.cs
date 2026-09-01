using Datasources;

namespace Datasources.Tests;

public class ToUpperCaseConverterTests
{
    [Fact]
    public void Uppers()
    {
        Assert.Equal("ADA", new ToUpperCaseConverter().Convert("ada"));
        Assert.Equal("ada", new ToUpperCaseConverter().ReverseConvert("ada"));
    }
}
