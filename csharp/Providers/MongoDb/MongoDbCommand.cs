using Datasources;

namespace Datasources.MongoDb;

internal sealed class MongoDbCommand : StubCommand
{
    public MongoDbCommand(IConnection connection) : base("mongodb", connection) { }
}
