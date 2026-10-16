using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresDropTriggerStatementRenderer : DropTriggerStatementTextRenderer
{
    public override string Render(DropTriggerStatement statement) =>
        $"DROP TRIGGER IF EXISTS {statement.Name} ON {statement.Table}";
}
