using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE TRIGGER {statement.Name} ON {statement.Table}
        AFTER UPDATE AS
        BEGIN
          SET NOCOUNT ON;
          UPDATE {statement.Table} SET {statement.UpdatedColumn} = {statement.UtcNow}
          FROM {statement.Table} t INNER JOIN inserted i ON t.{statement.KeyColumn} = i.{statement.KeyColumn};
        END
        """;
}
