using Datasources;

namespace Datasources.DynamoDb;

internal sealed class DynamoDbCommand : StubCommand
{
    public DynamoDbCommand(IConnection connection) : base("dynamodb", connection) { }
}
