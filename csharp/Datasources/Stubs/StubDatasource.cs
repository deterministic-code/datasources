namespace Datasources;

public sealed class StubDatasource : IDatasource
{
    IConnection? _connection;
    public StubDatasource(string provider) => Provider = provider;
    public string Provider { get; }
    IConnection DefaultConnection() => _connection ??= new StubConnection(Provider);
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public async Task<ICommand> CreateCommand(IConnection? connection = null)
    {
        if (connection is null)
        {
            connection = DefaultConnection();
            if (connection.IsOpen)
                throw new DatasourceException("Connection is already open");
            await connection.Open();
        }
        return new StubCommand(Provider, connection);
    }
    public ICRUDRepository<T> CreateRepository<T>(
        IConnection? connection = null,
        CRUDRepositoryOptions<T>? options = null) where T : new() =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
