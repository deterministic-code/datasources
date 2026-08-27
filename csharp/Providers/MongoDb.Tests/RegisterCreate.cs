using Datasources;
using Datasources.MongoDb;

public partial class RegisterCreateTests
{
    const string Provider = "mongodb";
    static readonly IDatasourceFactory Factory = MongoDbDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new MongoDbDatasource().CreateConnection();
        Assert.IsType<MongoDbConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
