using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerConnection : StubConnection
{
    public SqlServerConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("sqlserver") { }
    public override ICommand CreateCommand() => new SqlServerCommand(this);
    public static object Repository(IConnection connection) =>
        new StubRepository<object>(connection.Provider, connection);
}
