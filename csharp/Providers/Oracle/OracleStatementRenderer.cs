using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(CreateTableStatement create) =>
        new OracleCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new OracleAlterTableStatementRenderer().Render(alter);

    public override string Visit(DropIndexStatement index) =>
        new OracleDropIndexStatementRenderer().Render(index);

    public override string Visit(DropTriggerStatement trigger) =>
        new OracleDropTriggerStatementRenderer().Render(trigger);

    public override string Visit(CreateTriggerStatement trigger) =>
        new OracleCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(DropTableStatement drop) =>
        new OracleDropTableStatementRenderer().Render(drop);

    public override string Visit(CreateCreateStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateReadStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateNestedReadStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateUpdateStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateDeleteStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkCreateStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkUpdateStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkDeleteStoredProcedureStatement statement) =>
        new OracleStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(SqlSeedStatement seed) =>
        new OracleSqlSeedStatementRenderer().Render(seed);

    public override string Visit(CreateIndexStatement index) =>
        new OracleCreateIndexStatementRenderer().Render(index);

    public override string Visit(ForeignKeyStatement key) =>
        new OracleForeignKeyStatementRenderer().Render(key);

    public override IReadOnlyList<IStatement> Procedures(ProcedureSource source) =>
        SqlProcedureStatements.Of(source);
}
