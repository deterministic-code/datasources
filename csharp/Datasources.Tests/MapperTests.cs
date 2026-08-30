using Datasources;

namespace Datasources.Tests;

public class MapperTests
{
    [Fact]
    public void ToAndFromAreTheName()
    {
        var mapper = new Mapper("users", "sql", "csharp");
        Assert.Equal("users", mapper.Name);
        Assert.Equal("sql", mapper.Source);
        Assert.Equal("csharp", mapper.Destination);
        Assert.Equal("user_accounts", mapper.To("user_accounts"));
        Assert.Equal("UserAccounts", mapper.From("UserAccounts"));
    }
}
