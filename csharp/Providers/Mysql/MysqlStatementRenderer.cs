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

    public override string Visit(DropRoutineStatement routine) =>
        new MysqlDropRoutineStatementRenderer().Render(routine);

    public override string Visit(CreateCreateStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateReadStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateNestedReadStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateUpdateStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateDeleteStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkCreateStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkUpdateStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkDeleteStoredProcedureStatement statement) =>
        new MysqlStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(SqlSeedStatement seed) =>
        new MysqlSqlSeedStatementRenderer().Render(seed);

    public override string Visit(DropTableStatement drop) =>
        new MysqlDropTableStatementRenderer().Render(drop);

    public override string Visit(CreateIndexStatement index) =>
        new MysqlCreateIndexStatementRenderer().Render(index);

    public override string Visit(ForeignKeyStatement key) =>
        new MysqlForeignKeyStatementRenderer().Render(key);

    public override IReadOnlyList<IStatement> Procedures(ProcedureSource source) =>
        SqlProcedureStatements.Of(source);
}
