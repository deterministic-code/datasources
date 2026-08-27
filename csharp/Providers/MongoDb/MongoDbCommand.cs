using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbCommand : StubCommand
{
    public MongoDbCommand(IConnection connection) : base("mongodb", connection) { }
}
