using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlDatasource : IDatasourceFactory
{
    public static readonly MysqlDatasource Factory = new();
    public string Provider => "mysql";
    public IDatasource Create(object? options = null) => new StubDatasource("mysql");
}
