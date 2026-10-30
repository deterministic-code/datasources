using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement)
    {
        var table = SqlServerDriver.Quote(statement.Table);
        var updated = SqlServerDriver.Quote(statement.UpdatedColumn);
        var key = SqlServerDriver.Quote(statement.KeyColumn);
        return $"""
            CREATE TRIGGER {SqlServerDriver.Quote(statement.Name)} ON {table}
            AFTER UPDATE AS
            BEGIN
              SET NOCOUNT ON;
              UPDATE {table} SET {updated} = {SqlServerDriver.UtcNow()}
              FROM {table} t INNER JOIN inserted i ON t.{key} = i.{key};
            END
            """;
    }
}
