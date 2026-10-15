using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE OR REPLACE TRIGGER {statement.Name}
        BEFORE UPDATE ON {statement.Table}
        FOR EACH ROW
        BEGIN
          :NEW.{statement.UpdatedColumn} := {statement.UtcNow};
        END;
        /
        """;
}
