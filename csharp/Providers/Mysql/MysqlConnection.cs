using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlConnection : StubConnection
{
    public MysqlConnection() : base("mysql") { }
}
