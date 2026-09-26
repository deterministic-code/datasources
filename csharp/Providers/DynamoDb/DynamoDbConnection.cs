using Datasources;

namespace Datasources.DynamoDb;

internal sealed class DynamoDbConnection : StubConnection
{
    public DynamoDbConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("dynamodb") { }
}
