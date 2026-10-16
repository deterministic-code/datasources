using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new MysqlSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new MysqlCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new MysqlAlterTableStatementRenderer().Render(alter);

    public override string Visit(DropIndexStatement index) =>
        new MysqlDropIndexStatementRenderer().Render(index);

    public override string Visit(BulkUpdateStatement update) =>
        new MysqlBulkUpdateStatementTextRenderer().Render(update);

    public override string Visit(CreateTriggerStatement trigger) =>
        new MysqlCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(CreateRoutineStatement routine) =>
        new MysqlCreateRoutineStatementRenderer().Render(routine);

    public override string Visit(DropRoutineStatement routine) =>
        new MysqlDropRoutineStatementRenderer().Render(routine);
}
