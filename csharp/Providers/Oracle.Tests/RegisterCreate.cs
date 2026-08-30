using Datasources;
using Datasources.Oracle;

public partial class RegisterCreateTests
{
    const string Provider = "oracle";
    static readonly IDatasourceFactory Instance = OracleDatasource.Instance;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new OracleDatasource().CreateConnection();
        Assert.IsType<OracleConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new OracleDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<OracleCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(""));
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<OracleCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader(""));
    }
}
