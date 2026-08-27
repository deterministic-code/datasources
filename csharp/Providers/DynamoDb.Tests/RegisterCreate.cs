using Datasources;
using Datasources.DynamoDb;

public partial class RegisterCreateTests
{
    const string Provider = "dynamodb";
    static readonly IDatasourceFactory Factory = DynamoDbDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new DynamoDbDatasource().CreateConnection();
        Assert.IsType<DynamoDbConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
