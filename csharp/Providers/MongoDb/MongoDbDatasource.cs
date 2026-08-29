using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MongoDbDatasource Instance = new();
    public string Provider => "mongodb";
    public IDatasource Create(object? options = null) => new MongoDbDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new MongoDbConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new MongoDbCommand(connection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
