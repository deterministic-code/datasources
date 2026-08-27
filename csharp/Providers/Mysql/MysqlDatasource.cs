using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlConnection : StubConnection
{
    public MysqlConnection() : base("mysql") { }
}

public sealed class MysqlDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MysqlDatasource Factory = new();
    public string Provider => "mysql";
    public IDatasource Create(object? options = null) => new MysqlDatasource();
    IConnection NewConnection() => new MysqlConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
