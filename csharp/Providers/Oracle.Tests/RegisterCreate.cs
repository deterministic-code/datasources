using Datasources;
using Datasources.Oracle;

public partial class RegisterCreateTests
{
    const string Provider = "oracle";
    static readonly IDatasourceFactory Factory = OracleDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new OracleDatasource().CreateConnection();
        Assert.IsType<OracleConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
