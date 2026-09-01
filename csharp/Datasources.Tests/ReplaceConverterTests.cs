using Datasources;

namespace Datasources.Tests;

public class ReplaceConverterTests
{
    [Fact]
    public void ReplacesLiterals()
    {
        Assert.Equal("Ada Lovelace", new ReplaceConverter("_", " ").Convert("Ada_Lovelace"));
        Assert.Equal("Ada_Lovelace", new ReplaceConverter("_", " ").ReverseConvert("Ada Lovelace"));
    }
}
