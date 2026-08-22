using Datasources;
using Datasources.Oracle;

namespace Datasources.Oracle.Tests;

public class OracleTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(OracleProvider.Factory);
        Assert.Equal("oracle", OracleProvider.Factory.Provider);
        var ds = Datasource.Create("oracle");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
