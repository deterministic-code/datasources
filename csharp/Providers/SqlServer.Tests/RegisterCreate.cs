using Datasources;
using Datasources.SqlServer;

public partial class RegisterCreateTests
{
    const string Provider = "sqlserver";
    static readonly IDatasourceFactory Instance = SqlServerDatasource.Instance;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new SqlServerDatasource().CreateConnection();
        Assert.IsType<SqlServerConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new SqlServerDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<SqlServerCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(""));
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<SqlServerCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader(""));
    }
}
