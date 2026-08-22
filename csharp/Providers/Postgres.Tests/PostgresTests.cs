using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(PostgresProvider.Factory);
        Assert.Equal("postgres", PostgresProvider.Factory.Provider);
        var ds = Datasource.Create("postgres");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
