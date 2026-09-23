using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCommand : StubCommand
{
    public SqlServerCommand(IConnection connection) : base("sqlserver", connection) { }
}
