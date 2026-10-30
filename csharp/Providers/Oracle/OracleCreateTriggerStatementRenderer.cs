using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE OR REPLACE TRIGGER {OracleDriver.Quote(statement.Name)}
        BEFORE UPDATE ON {OracleDriver.Quote(statement.Table)}
        FOR EACH ROW
        BEGIN
          :NEW.{OracleDriver.Quote(statement.UpdatedColumn)} := {OracleDriver.UtcNow()};
        END;
        /
        """;
}
