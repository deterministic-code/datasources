using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbConnection : StubConnection
{
    public DynamoDbConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("dynamodb") { }
    public override ICommand CreateCommand() => new DynamoDbCommand(this);
    public static object Repository(IConnection connection) =>
        new StubRepository<object>(connection.Provider, connection);
}
