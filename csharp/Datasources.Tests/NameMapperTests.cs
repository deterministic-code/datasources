using Datasources;

namespace Datasources.Tests;

public class NameMapperTests
{
    [Fact]
    public void MapAndReverseMapAreTheName()
    {
        INameMapper mapping = new NameSourceDestinationMapper("sql", "csharp");
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }

    [Fact]
    public void SourceAndDestination()
    {
        var mapper = new NameSourceDestinationMapper("sql", "csharp");
        Assert.Equal("sql", mapper.Source);
        Assert.Equal("csharp", mapper.Destination);
        INameMapper mapping = mapper;
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }
}
