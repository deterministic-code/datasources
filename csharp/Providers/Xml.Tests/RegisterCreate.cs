using Datasources;
using Datasources.Xml;

public partial class RegisterCreateTests
{
    const string Provider = "xml";
    static readonly IDriver Driver = new XmlDriver();
    static IDatasource Ds() => new Datasource(Provider, new XmlDriver());

    [Fact]
    public async Task CreatesConnection()
    {
        var connection = await Ds().CreateConnection();
        Assert.IsType<XmlConnection>(connection);
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
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => ds.CreateCommand());
    }
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(XmlDriver), Assert.Single(typeof(XmlDriver).Assembly.GetExportedTypes()));
}
