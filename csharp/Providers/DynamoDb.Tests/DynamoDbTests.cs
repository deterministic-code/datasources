using Datasources;
using Datasources.DynamoDb;

namespace Datasources.DynamoDb.Tests;

public class DynamoDbTests
{
    [Fact]
    public async Task RegistersAndThrows()
    {
        Datasource.Reset();
        Datasource.Use(DynamoDbProvider.Factory);
        Assert.Equal("dynamodb", DynamoDbProvider.Factory.Provider);
        var ds = Datasource.Create("dynamodb");
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateConnection().Open());
    }
}
