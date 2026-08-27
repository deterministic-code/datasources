using Datasources;

namespace Datasources.Mysql;

public sealed class MysqlCommand : StubCommand
{
    public MysqlCommand(IConnection connection) : base("mysql", connection) { }
}
