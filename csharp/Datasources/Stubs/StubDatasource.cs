namespace Datasources;

public sealed class StubDatasource : IDatasource
{
    IConnection? _connection;
    public StubDatasource(string provider) => Provider = provider;
    public string Provider { get; }
    IConnection DefaultConnection() => _connection ??= new StubConnection(Provider);
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? DefaultConnection());
    public ICRUDRepository<T> CreateRepository<T>(
        IConnection? connection = null,
        CRUDRepositoryOptions<T>? options = null) where T : new() =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
