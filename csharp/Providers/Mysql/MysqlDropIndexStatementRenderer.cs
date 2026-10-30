using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    public override string Render(DropIndexStatement statement) =>
        $"DROP INDEX {MysqlDriver.Quote(statement.Name)} ON {MysqlDriver.Quote(statement.Table)}";
}
