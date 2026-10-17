using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerStatementRenderer(StatementTextRenderer text) : StatementRenderer(text)
{
    public override string Visit(CreateTableStatement create) =>
        new SqlServerCreateTableStatementRenderer().Render(create);

    public override string Visit(AlterTableStatement alter) =>
        new SqlServerAlterTableStatementRenderer().Render(alter);

    public override string Visit(DropIndexStatement index) =>
        new SqlServerDropIndexStatementRenderer().Render(index);

    public override string Visit(CreateTriggerStatement trigger) =>
        new SqlServerCreateTriggerStatementRenderer().Render(trigger);

    public override string Visit(IdentityInsertStatement insert) =>
        new SqlServerIdentityInsertStatementRenderer().Render(insert);

    public override string Visit(DropRoutineStatement routine) =>
        new SqlServerDropRoutineStatementRenderer().Render(routine);

    public override string Visit(CreateCreateStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateReadStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateUpdateStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateDeleteStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkCreateStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkUpdateStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);

    public override string Visit(CreateBulkDeleteStoredProcedureStatement statement) =>
        new SqlServerStoredProcedureStatementRenderer().Render(statement);
}
