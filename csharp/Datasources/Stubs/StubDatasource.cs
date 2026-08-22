namespace Datasources;

public sealed class StubDatasource : IDatasource, IDatasourceFactory
{
    public StubDatasource(string provider) => Provider = provider;
    public string Provider { get; }
    public IDatasource Create(object? options = null) => new StubDatasource(Provider);
    public IConnection CreateConnection() => new StubConnection(Provider);
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? CreateConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? CreateConnection());
}
