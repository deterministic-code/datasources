using Datasources;

namespace Datasources.MongoDb;

public static class MongoDbProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("mongodb");

    static MongoDbProvider()
    {
        Datasource.Use(Factory);
    }
}
