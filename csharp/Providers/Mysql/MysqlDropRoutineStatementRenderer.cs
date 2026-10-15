using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlDropRoutineStatementRenderer : DropRoutineStatementTextRenderer
{
    public override string Render(DropRoutineStatement statement) =>
        $"DROP PROCEDURE IF EXISTS {statement.Name}";
}
