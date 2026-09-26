using Datasources;

namespace Datasources.MongoDb;

internal sealed class MongoDbConnection : StubConnection
{
    public MongoDbConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("mongodb") { }
}
