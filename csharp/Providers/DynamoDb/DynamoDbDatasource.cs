using Datasources;

namespace Datasources.DynamoDb;

public static class DynamoDbProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("dynamodb");

    static DynamoDbProvider()
    {
        Datasource.Use(Factory);
    }
}
