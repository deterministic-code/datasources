using Datasources;

namespace Datasources.Tests;

public class PluralizerMapperTests
{
    [Fact]
    public void PluralizesAndLowercasesTheTableName()
    {
        var mapper = new PluralizerMapper();
        Assert.Equal("users", mapper.Map("User"));
        Assert.Equal("user", mapper.ReverseMap("users"));
    }
}
