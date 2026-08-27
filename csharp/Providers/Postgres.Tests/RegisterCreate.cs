using Datasources;
using Datasources.Postgres;

public partial class RegisterCreateTests
{
    const string Provider = "postgres";
    static readonly IDatasourceFactory Factory = PostgresDatasource.Factory;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new PostgresDatasource().CreateConnection();
        Assert.IsType<PostgresConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }
}
