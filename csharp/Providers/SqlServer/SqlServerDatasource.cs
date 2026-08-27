using Datasources;

namespace Datasources.SqlServer;

public sealed class SqlServerDatasource : IDatasourceFactory
{
    public static readonly SqlServerDatasource Factory = new();
    public string Provider => "sqlserver";
    public IDatasource Create(object? options = null) => new StubDatasource("sqlserver");
}
