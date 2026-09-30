using Datasources;

namespace Datasources.Tests;

public class IdentityMapperTests
{
    [Fact]
    public void MapsAndReversesTheSameValue()
    {
        var mapper = new IdentityMapper<string>();
        var mapped = mapper.Map("contact");
        var reversed = mapper.ReverseMap("contact");
        Assert.Equal("contact", mapped);
        Assert.Equal("contact", reversed);
    }
}
