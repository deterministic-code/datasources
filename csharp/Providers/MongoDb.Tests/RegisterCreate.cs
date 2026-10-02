using Datasources;
using Datasources.MongoDb;

public partial class RegisterCreateTests
{
    const string Provider = "mongodb";
    static readonly IDriver Driver = new MongoDbDriver();
    static IDatasource Ds() => new Datasource(Provider, new MongoDbDriver());

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await Ds().CreateConnection();
        Assert.IsType<MongoDbConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = Ds();
        var connection = await ds.CreateConnection();
        var command = await ds.CreateCommand(connection);
        Assert.IsType<StubCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(Native.N("x")));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(Native.N("x")));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateCommand());
    }
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(MongoDbDriver), Assert.Single(typeof(MongoDbDriver).Assembly.GetExportedTypes()));
}
