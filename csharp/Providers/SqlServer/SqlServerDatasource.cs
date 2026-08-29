using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerDatasource : IDatasource, IDatasourceFactory
{
    public static readonly SqlServerDatasource Instance = new();
    public string Provider => "sqlserver";
    public IDatasource Create(object? options = null) => new SqlServerDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new SqlServerConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new SqlServerCommand(connection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
