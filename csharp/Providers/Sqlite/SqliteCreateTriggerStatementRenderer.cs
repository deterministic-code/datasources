using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE TRIGGER {statement.Name}
        AFTER UPDATE ON {statement.Table}
        FOR EACH ROW
        WHEN NEW.{statement.UpdatedColumn} IS OLD.{statement.UpdatedColumn}
        BEGIN
          UPDATE {statement.Table} SET {statement.UpdatedColumn} = {statement.UtcNow} WHERE {statement.KeyColumn} = OLD.{statement.KeyColumn};
        END
        """;
}
