using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerDropRoutineStatementRenderer : DropRoutineStatementTextRenderer
{
    public override string Render(DropRoutineStatement statement) =>
        $"DROP PROCEDURE IF EXISTS {statement.Name}";
}
