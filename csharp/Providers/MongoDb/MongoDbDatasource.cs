using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbConnection : StubConnection
{
    public MongoDbConnection() : base("mongodb") { }
}

public sealed class MongoDbDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MongoDbDatasource Factory = new();
    public string Provider => "mongodb";
    public IDatasource Create(object? options = null) => new MongoDbDatasource();
    IConnection NewConnection() => new MongoDbConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
