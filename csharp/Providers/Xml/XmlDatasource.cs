using Datasources;

namespace Datasources.Xml;

public sealed class XmlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly XmlDatasource Factory = new();
    public string Provider => "xml";
    public IDatasource Create(object? options = null) => new XmlDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new XmlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new XmlCommand(connection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
