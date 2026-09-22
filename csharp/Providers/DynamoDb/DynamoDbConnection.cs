using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbConnection : StubConnection
{
    public DynamoDbConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("dynamodb") { }
    public override ICommand CreateCommand() => new DynamoDbCommand(this);

}
