using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerConnection : StubConnection
{
    public SqlServerConnection() : base("sqlserver") { }
}
