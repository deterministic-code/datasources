using Datasources;
using Datasources.Mysql;

public partial class RegisterCreateTests
{
    const string Provider = "mysql";
    static readonly IDatasourceFactory Factory = MysqlDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new MysqlDatasource().CreateConnection();
        Assert.IsType<MysqlConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
