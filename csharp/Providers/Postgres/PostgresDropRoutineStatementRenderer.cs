using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresDropRoutineStatementRenderer : DropRoutineStatementTextRenderer
{
    public override string Render(DropRoutineStatement statement) =>
        $"DROP FUNCTION IF EXISTS {statement.Name}";
}
