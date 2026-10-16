using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    public override string Render(DropIndexStatement statement) =>
        $"DROP INDEX {statement.Name} ON {statement.Table}";
}
