using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE TRIGGER {statement.Name} BEFORE UPDATE ON {statement.Table}
        FOR EACH ROW SET NEW.{statement.UpdatedColumn} = {statement.UtcNow}
        """;
}
