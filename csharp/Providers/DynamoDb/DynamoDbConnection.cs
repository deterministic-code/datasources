using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbConnection : StubConnection
{
    public DynamoDbConnection() : base("dynamodb") { }
}
