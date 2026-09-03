namespace Datasources;

public sealed class StubDatasource : IDatasource, IDatasourceFactory
{
    IConnection? _connection;
    public StubDatasource(string provider) => Provider = provider;
    public string Provider { get; }
    public IDatasource Create(object? options = null) => new StubDatasource(Provider);
    IConnection DefaultConnection() => _connection ??= new StubConnection(Provider);
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? DefaultConnection());
    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
