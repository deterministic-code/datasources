using Datasources;

namespace Datasources.Tests;

public class RegexConverterTests
{
    [Fact]
    public void ReplacesWithRegex()
    {
        Assert.Equal("Ada  Lovelace", new RegexConverter(@"\d+", "").Convert("Ada 1 Lovelace"));
        Assert.Equal("Ada  Lovelace", new RegexConverter(new System.Text.RegularExpressions.Regex(@"\d+"), "").Convert("Ada 1 Lovelace"));
        Assert.Equal("id-7", new RegexConverter(@"id_(\d+)", "id-$1").Convert("id_7"));
        Assert.Equal("id_7", new RegexConverter(@"id_(\d+)", "id-$1").ReverseConvert("id_7"));
    }
}
