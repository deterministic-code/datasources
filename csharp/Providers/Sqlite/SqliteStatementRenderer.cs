using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new SqliteSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new SqliteCreateTableStatementRenderer().Render(create);
}
