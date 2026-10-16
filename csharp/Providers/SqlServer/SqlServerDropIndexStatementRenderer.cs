using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    public override string Render(DropIndexStatement statement) =>
        $"DROP INDEX {statement.Name} ON {statement.Table}";
}
