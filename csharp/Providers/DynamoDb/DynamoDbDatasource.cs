using Datasources;

namespace Datasources.DynamoDb;

public sealed class DynamoDbDatasource : IDatasourceFactory
{
    public static readonly DynamoDbDatasource Factory = new();
    public string Provider => "dynamodb";
    public IDatasource Create(object? options = null) => new StubDatasource("dynamodb");
}
