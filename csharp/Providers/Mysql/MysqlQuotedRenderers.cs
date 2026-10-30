using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlDropTableStatementRenderer : DropTableStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}

internal sealed class MysqlCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}
