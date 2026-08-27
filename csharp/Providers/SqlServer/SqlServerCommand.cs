using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerCommand : StubCommand
{
    public SqlServerCommand(IConnection connection) : base("sqlserver", connection) { }
}
