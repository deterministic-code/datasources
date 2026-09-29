using Datasources;
using Datasources.Oracle;

public partial class RegisterCreateTests
{
    const string Provider = "oracle";
    static readonly IDriver Driver = new OracleDriver();
    static IDatasource Ds() => new Datasource(Provider, new OracleDriver());

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await Ds().CreateConnection();
        Assert.IsType<OracleConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = Ds();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<StubCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(""));
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<StubCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader(""));
    }
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(OracleDriver), Assert.Single(typeof(OracleDriver).Assembly.GetExportedTypes()));
}
