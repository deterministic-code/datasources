namespace Datasources;

public sealed class MockDatasource : IDatasource
{
    public int CreateConnectionCalls { get; private set; }
    public IConnection Connection { get; set; }
    public ICommand? Command { get; set; }
    public object? Repository { get; set; }
    public string Provider { get; }

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

    public ICommand CreateCommand(IConnection? connection = null) =>
        Command ?? new MockCommand(connection ?? Connection);

    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) where T : new() =>
        Repository as ICRUDRepository<T> ?? new MockRepository<T>(connection ?? Connection);
}
