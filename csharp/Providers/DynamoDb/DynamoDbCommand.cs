using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbCommand : StubCommand
{
    public DynamoDbCommand(IConnection connection) : base("dynamodb", connection) { }
}
