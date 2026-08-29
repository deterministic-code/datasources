using Datasources;
using Datasources.Yaml;

public partial class RegisterCreateTests
{
    const string Provider = "yaml";
    static readonly IDatasourceFactory Instance = YamlDatasource.Instance;

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await new YamlDatasource().CreateConnection();
        Assert.IsType<YamlConnection>(connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(connection.Open);
    }

    [Fact]
    public async Task CreatesCommand()
    {
        var ds = new YamlDatasource();
        var connection = await ds.CreateConnection();
        var command = ds.CreateCommand(connection);
        Assert.IsType<YamlCommand>(command);
        Assert.Same(connection, command.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader());
        var implicitCommand = ds.CreateCommand();
        Assert.IsType<YamlCommand>(implicitCommand);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => implicitCommand.ExecuteReader());
    }
}
