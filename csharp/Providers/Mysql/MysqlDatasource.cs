using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlConnection : StubConnection
{
    public MysqlConnection() : base("mysql") { }
}

public sealed class MysqlCommand : StubCommand
{
    public MysqlCommand(IConnection connection) : base("mysql", connection) { }
}

public sealed class MysqlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MysqlDatasource Factory = new();
    public string Provider => "mysql";
    public IDatasource Create(object? options = null) => new MysqlDatasource();
    IConnection NewConnection() => new MysqlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new MysqlCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
