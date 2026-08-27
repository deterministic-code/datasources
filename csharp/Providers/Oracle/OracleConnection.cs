using Datasources;

namespace Datasources.Oracle;

public sealed class OracleConnection : StubConnection
{
    public OracleConnection() : base("oracle") { }
}
