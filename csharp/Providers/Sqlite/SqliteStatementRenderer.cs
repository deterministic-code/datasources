using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new SqliteSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new SqliteCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new SqliteAlterTableStatementRenderer().Render(alter);

    public override string Visit(CreateTriggerStatement trigger) =>
        new SqliteCreateTriggerStatementRenderer().Render(trigger);
}
