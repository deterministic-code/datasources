namespace Datasources;

public sealed class StubDatasource : IDatasource, IDatasourceFactory
{
    public StubDatasource(string provider) => Provider = provider;
    public string Provider { get; }
    public IDatasource Create(object? options = null) => new StubDatasource(Provider);
    IConnection NewConnection() => new StubConnection(Provider);
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
