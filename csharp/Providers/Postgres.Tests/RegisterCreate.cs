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

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new PostgresDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<PostgresCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader());
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<PostgresCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader());
    }
}
