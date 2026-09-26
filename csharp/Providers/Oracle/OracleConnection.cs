using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleConnection : StubConnection
{
    public OracleConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("oracle") { }
}
