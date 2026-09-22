using Datasources;

namespace Datasources.MongoDb;

public sealed class MongoDbConnection : StubConnection
{
    public MongoDbConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("mongodb") { }
    public override ICommand CreateCommand() => new MongoDbCommand(this);

}
