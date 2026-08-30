using Datasources;

namespace Datasources.Tests;

public class NameMapperTests
{
    [Fact]
    public void MapAndReverseMapAreTheName()
    {
        IMapper<string, string> mapping = new NameSourceDestinationMapper("sql", "csharp");
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }

    [Fact]
    public void SourceAndDestination()
    {
        var mapper = new NameSourceDestinationMapper("sql", "csharp");
        ISourceDestinationMapper labels = mapper;
        Assert.Equal("sql", labels.Source);
        Assert.Equal("csharp", labels.Destination);
        IMapper<string, string> mapping = mapper;
        Assert.Equal("user_accounts", mapping.Map("user_accounts"));
        Assert.Equal("UserAccounts", mapping.ReverseMap("UserAccounts"));
    }
}
