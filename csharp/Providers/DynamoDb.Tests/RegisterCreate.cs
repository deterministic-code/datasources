using Datasources;
using Datasources.DynamoDb;

public partial class RegisterCreateTests
{
    const string Provider = "dynamodb";
    static readonly IDriver Driver = new DynamoDbProvider();
    static IDatasource Ds() => new Datasource(Provider, new DynamoDbProvider());

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await Ds().CreateConnection();
        Assert.IsType<DynamoDbConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = Ds();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<DynamoDbCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(""));
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<DynamoDbCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader(""));
    }
}
