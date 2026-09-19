namespace Datasources;

public sealed class Datasource(
    string provider,
    Func<IConnection> newConnection,
    Func<IConnection, object> newRepository) : IDatasource
{
    IConnection? _connection;
    public string Provider { get; } = provider;
    IConnection DefaultConnection() => _connection ??= newConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        (connection ?? DefaultConnection()).CreateCommand();
    public ICRUDRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        (ICRUDRepository<T>)newRepository(connection ?? DefaultConnection());
}
