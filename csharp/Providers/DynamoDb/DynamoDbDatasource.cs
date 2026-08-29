using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbDatasource : IDatasource, IDatasourceFactory
{
    public static readonly DynamoDbDatasource Instance = new();
    public string Provider => "dynamodb";
    public IDatasource Create(object? options = null) => new DynamoDbDatasource();
    IConnection? _connection;
    IConnection DefaultConnection() => _connection ??= new DynamoDbConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(DefaultConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new DynamoDbCommand(connection ?? DefaultConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? DefaultConnection());
}
