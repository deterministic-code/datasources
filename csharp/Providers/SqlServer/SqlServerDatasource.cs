using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerConnection : StubConnection
{
    public SqlServerConnection() : base("sqlserver") { }
}

public sealed class SqlServerCommand : StubCommand
{
    public SqlServerCommand(IConnection connection) : base("sqlserver", connection) { }
}

public sealed class SqlServerDatasource : IDatasource, IDatasourceFactory
{
    public static readonly SqlServerDatasource Factory = new();
    public string Provider => "sqlserver";
    public IDatasource Create(object? options = null) => new SqlServerDatasource();
    IConnection NewConnection() => new SqlServerConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new SqlServerCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
