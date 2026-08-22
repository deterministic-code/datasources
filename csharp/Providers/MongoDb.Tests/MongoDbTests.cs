using Datasources;
using Datasources.MongoDb;

namespace Datasources.MongoDb.Tests;

public class MongoDbTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(MongoDbProvider.Factory);
        Assert.Equal("mongodb", MongoDbProvider.Factory.Provider);
        var ds = Datasource.Create("mongodb");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
