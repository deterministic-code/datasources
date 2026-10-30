using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlCreateTriggerStatementRenderer : CreateTriggerStatementTextRenderer
{
    public override string Render(CreateTriggerStatement statement) =>
        $"""
        CREATE TRIGGER {MysqlDriver.Quote(statement.Name)} BEFORE UPDATE ON {MysqlDriver.Quote(statement.Table)}
        FOR EACH ROW SET NEW.{MysqlDriver.Quote(statement.UpdatedColumn)} = {MysqlDriver.UtcNow()}
        """;
}
