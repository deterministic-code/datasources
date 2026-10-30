using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement)
    {
        var table = SqliteDriver.Quote(statement.Table);
        var name = SqliteDriver.Quote(statement.Name);
        var updated = SqliteDriver.Quote(statement.UpdatedColumn);
        var key = SqliteDriver.Quote(statement.KeyColumn);
        return $"""
            CREATE TRIGGER {name}
            AFTER UPDATE ON {table}
            FOR EACH ROW
            WHEN NEW.{updated} IS OLD.{updated}
            BEGIN
              UPDATE {table} SET {updated} = {SqliteDriver.UtcNow()} WHERE {key} = OLD.{key};
            END
            """;
    }
}
