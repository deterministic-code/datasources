using Datasources;
using Datasources.SqlServer;

namespace Datasources.SqlServer.Tests;

public class SqlServerTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(SqlServerProvider.Factory);
        Assert.Equal("sqlserver", SqlServerProvider.Factory.Provider);
        var ds = Datasource.Create("sqlserver");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
