namespace Datasources;

public sealed class MockDatasource : IDatasource
{
    public int CreateConnectionCalls { get; private set; }
    public IConnection Connection { get; set; }
    public ICommand? Command { get; set; }
    public object? Repository { get; set; }
    public string Provider { get; }
    public RepositoryFactory Repositories { get; } = new();

    public MockDatasource(string provider = "mock")
    {
        Provider = provider;
        Connection = new MockConnection();
    }

    public Task<IConnection> CreateConnection()
    {
        CreateConnectionCalls++;
        return Task.FromResult(Connection);
    }

    public async Task<ICommand> CreateCommand(IConnection? connection = null)
    {
        if (connection is null)
        {
            if (Connection.IsOpen)
                throw new DatasourceException("Connection is already open");
            await Connection.Open();
        }
        return Command ?? new MockCommand(connection ?? Connection);
    }

    public ICRUDRepository<T> CreateRepository<T>(
        IConnection? connection = null,
        CRUDRepositoryOptions<T>? options = null) where T : new() =>
        Repository as ICRUDRepository<T> ?? new MockRepository<T>(connection ?? Connection);

    public object CreateRepository(string typeName, IConnection? connection = null) =>
        Repositories.Create(typeName, connection ?? Connection);

    public object CreateReadonlyRepository(string typeName, IConnection? connection = null) =>
        Repositories.CreateReadonly(typeName, connection ?? Connection);
}
