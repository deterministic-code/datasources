using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbConnection : StubConnection
{
    public DynamoDbConnection() : base("dynamodb") { }
}

public sealed class DynamoDbDatasource : IDatasource, IDatasourceFactory
{
    public static readonly DynamoDbDatasource Factory = new();
    public string Provider => "dynamodb";
    public IDatasource Create(object? options = null) => new DynamoDbDatasource();
    IConnection NewConnection() => new DynamoDbConnection();
    public Task<IConnection> CreateConnection() => Task.FromResult(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new StubCommand(Provider, connection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null) =>
        new StubRepository<T>(Provider, connection ?? NewConnection());
}
