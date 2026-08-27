using Datasources;
using Datasources.SqlServer;

public partial class RegisterCreateTests
{
    const string Provider = "sqlserver";
    static readonly IDatasourceFactory Factory = SqlServerDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new SqlServerDatasource().CreateConnection();
        Assert.IsType<SqlServerConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
