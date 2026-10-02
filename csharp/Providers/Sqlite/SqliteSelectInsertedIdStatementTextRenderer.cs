using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteSelectInsertedIdStatementTextRenderer : SelectInsertedIdStatementTextRenderer
{
    public override string Render(SelectInsertedIdStatement statement) =>
        $"SELECT last_insert_rowid() AS {statement.Column}";
}
