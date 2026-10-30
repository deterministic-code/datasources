using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    public override string Render(DropIndexStatement statement) =>
        $"DROP INDEX {SqlServerDriver.Quote(statement.Name)} ON {SqlServerDriver.Quote(statement.Table)}";
}
