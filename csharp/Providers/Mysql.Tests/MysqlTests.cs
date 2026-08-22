using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(MysqlProvider.Factory);
        Assert.Equal("mysql", MysqlProvider.Factory.Provider);
        var ds = Datasource.Create("mysql");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
