using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbDatasource : IDatasourceFactory
{
    public static readonly MongoDbDatasource Factory = new();
    public string Provider => "mongodb";
    public IDatasource Create(object? options = null) => new StubDatasource("mongodb");
}
