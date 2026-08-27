using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbConnection : StubConnection
{
    public MongoDbConnection() : base("mongodb") { }
}
