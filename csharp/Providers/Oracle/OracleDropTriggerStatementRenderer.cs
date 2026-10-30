using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleDropTriggerStatementRenderer : DropTriggerStatementTextRenderer
{
    public override string Render(DropTriggerStatement statement) =>
        $"DROP TRIGGER {OracleDriver.Quote(statement.Name)}";
}
