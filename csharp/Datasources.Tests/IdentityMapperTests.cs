using Datasources;

namespace Datasources.Tests;

public class IdentityMapperTests
{
    [Fact]
    public void MapsAndReversesTheSameValue()
    {
        var mapper = new IdentityMapper<string>();
        Assert.Equal("ada", mapper.Map("ada"));
        Assert.Equal("ada", mapper.ReverseMap("ada"));
    }
}
