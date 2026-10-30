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

    public override string Visit(SqlSeedStatement seed) =>
        new SqliteSqlSeedStatementRenderer().Render(seed);

    public override string Visit(DropTableStatement drop) =>
        new SqliteDropTableStatementRenderer().Render(drop);

    public override string Visit(CreateIndexStatement index) =>
        new SqliteCreateIndexStatementRenderer().Render(index);

    public override string Visit(DropIndexStatement index) =>
        new SqliteDropIndexStatementRenderer().Render(index);

    public override string Visit(DropTriggerStatement trigger) =>
        new SqliteDropTriggerStatementRenderer().Render(trigger);

    public override string Visit(ForeignKeyStatement key) =>
        new SqliteForeignKeyStatementRenderer().Render(key);
}
