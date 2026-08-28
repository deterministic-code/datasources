using Datasources;

namespace Datasources.Tests;

public class MapperTests
{
    [Fact]
    public void MapsNamesTheCallerSupplies()
    {
        var mapping = DatasourceMapper.Mapping("sql", "csharp", new Dictionary<string, string>
        {
            ["user_accounts"] = "UserAccounts",
            ["created_at"] = "CreatedAt"
        });
        Assert.Equal("sql", mapping.Source);
        Assert.Equal("csharp", mapping.Destination);
        Assert.Equal("UserAccounts", mapping.To("user_accounts"));
        Assert.Equal("id", mapping.To("id"));
        Assert.Equal("created_at", mapping.From("CreatedAt"));
        Assert.Equal("id", mapping.From("id"));
    }

    [Fact]
    public void IsIdentityWhenNoNamesAreGiven()
    {
        var mapping = DatasourceMapper.Mapping("sql", "go");
        Assert.Equal("user_name", mapping.To("user_name"));
        Assert.Equal("UserName", mapping.From("UserName"));
    }
}
