using Datasources;

namespace Datasources.Xml;

public sealed class XmlConnection : StubConnection
{
    public XmlConnection() : base("xml") { }
}

public sealed class XmlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly XmlDatasource Factory = new();
    public string Provider => "xml";
    public IDatasource Create(object? options = null) => new XmlDatasource();
    IConnection NewConnection() => new XmlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
