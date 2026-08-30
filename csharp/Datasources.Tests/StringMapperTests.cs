using Datasources;

namespace Datasources.Tests;

public class StringMapperTests
{
    [Fact]
    public void ToAndFromAreTheName()
    {
        IMapper<string> mapping = new StringMapper();
        Assert.Equal("user_accounts", mapping.To("user_accounts"));
        Assert.Equal("UserAccounts", mapping.From("UserAccounts"));
    }

    [Fact]
    public void SourceAndDestination()
    {
        var mapper = new StringSourceDestinationMapper("sql", "csharp");
        ISourceDestinationMapper labels = mapper;
        Assert.Equal("sql", labels.Source);
        Assert.Equal("csharp", labels.Destination);
        IMapper<string> mapping = mapper;
        Assert.Equal("user_accounts", mapping.To("user_accounts"));
        Assert.Equal("UserAccounts", mapping.From("UserAccounts"));
    }
}
