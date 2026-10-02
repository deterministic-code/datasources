using Datasources;
using Datasources.Yaml;

public partial class RegisterCreateTests
{
    const string Provider = "yaml";
    static readonly IDriver Driver = new YamlDriver();
    static IDatasource Ds() => new Datasource(Provider, new YamlDriver());

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await Ds().CreateConnection();
        Assert.IsType<YamlConnection>(connection);
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
        Assert.Equal(typeof(YamlDriver), Assert.Single(typeof(YamlDriver).Assembly.GetExportedTypes()));
}
