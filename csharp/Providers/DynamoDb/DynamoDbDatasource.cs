using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbDatasource : IDatasource, IDatasourceFactory
{
    public static readonly DynamoDbDatasource Factory = new();
    public string Provider => "dynamodb";
    public IDatasource Create(object? options = null) => new DynamoDbDatasource();
    IConnection NewConnection() => new DynamoDbConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new DynamoDbCommand(connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
