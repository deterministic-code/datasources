using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlSelectInsertedIdStatementTextRenderer : SelectInsertedIdStatementTextRenderer
{
    public override string Render(SelectInsertedIdStatement statement) =>
        $"SELECT LAST_INSERT_ID() AS {statement.Column}";
}
