namespace Datasources;

public sealed class MockDatasource : IDatasource, IDatasourceFactory
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

    public IDatasource Create(object? options = null) => new MockDatasource(Provider);
    public IConnection CreateConnection()
    {
        CreateConnectionCalls++;
        return Connection;
    }

    public ICommand CreateCommand(IConnection? connection = null) =>
        Command ?? new MockCommand(connection ?? Connection);

    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        Repository as IRepository<T> ?? new MockRepository<T>(connection ?? Connection);
}
