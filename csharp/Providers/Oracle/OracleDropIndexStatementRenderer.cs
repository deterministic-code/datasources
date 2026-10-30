using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleDropIndexStatementRenderer : DropIndexStatementTextRenderer
{
    public override string Render(DropIndexStatement statement) =>
        $"DROP INDEX {OracleDriver.Quote(statement.Name)}";
}
