using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(SelectInsertedIdStatement inserted) =>
        new PostgresSelectInsertedIdStatementTextRenderer().Render(inserted);

    public override string Visit(CreateTableStatement create) =>
        new PostgresCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new PostgresAlterTableStatementRenderer().Render(alter);

    public override string Visit(DropTriggerStatement trigger) =>
        new PostgresDropTriggerStatementRenderer().Render(trigger);

    public override string Visit(BulkUpdateStatement update) =>
        new PostgresBulkUpdateStatementTextRenderer().Render(update);

    public override string Visit(CreateTriggerStatement trigger) =>
        new PostgresCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(IdentityInsertStatement insert) =>
        new PostgresIdentityInsertStatementRenderer().Render(insert);

    public override string Visit(CreateRoutineStatement routine) =>
        new PostgresCreateRoutineStatementRenderer().Render(routine);

    public override string Visit(DropRoutineStatement routine) =>
        new PostgresDropRoutineStatementRenderer().Render(routine);
}
