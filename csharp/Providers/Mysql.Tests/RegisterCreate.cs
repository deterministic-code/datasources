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

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new MysqlDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<MysqlCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader());
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<MysqlCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader());
    }
}
