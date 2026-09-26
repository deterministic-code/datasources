using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerConnection : StubConnection
{
    public SqlServerConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("sqlserver") { }
}
