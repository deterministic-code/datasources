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

    public override string Visit(DropRoutineStatement routine) =>
        new PostgresDropRoutineStatementRenderer().Render(routine);

    public override string Visit(CreateCreateStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateReadStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateNestedReadStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateUpdateStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateDeleteStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkCreateStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkUpdateStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkDeleteStoredProcedureStatement statement) =>
        new PostgresStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(SqlSeedStatement seed) =>
        new PostgresSqlSeedStatementRenderer().Render(seed);

    public override string Visit(DropTableStatement drop) =>
        new PostgresDropTableStatementRenderer().Render(drop);

    public override string Visit(CreateIndexStatement index) =>
        new PostgresCreateIndexStatementRenderer().Render(index);

    public override string Visit(DropIndexStatement index) =>
        new PostgresDropIndexStatementRenderer().Render(index);

    public override string Visit(ForeignKeyStatement key) =>
        new PostgresForeignKeyStatementRenderer().Render(key);

    public override IReadOnlyList<IStatement> Procedures(ProcedureSource source) =>
        SqlProcedureStatements.Of(source);
}
