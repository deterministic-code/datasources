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

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new MongoDbDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<MongoDbCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader());
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<MongoDbCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader());
    }
}
