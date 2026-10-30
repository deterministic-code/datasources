namespace Datasources;

public class StatementRenderer(StatementTextRenderer text) : IStatementVisitor
{
    protected StatementTextRenderer Text => text;

    public virtual string Visit(SelectStatement statement) =>
        new SelectStatementTextRenderer(Text).Render(statement);

    public virtual string Visit(InsertStatement statement) =>
        new InsertStatementTextRenderer().Render(statement);

    public virtual string Visit(UpdateStatement statement) =>
        new UpdateStatementTextRenderer(Text).Render(statement);

    public virtual string Visit(DeleteStatement statement) =>
        new DeleteStatementTextRenderer(Text).Render(statement);

    public virtual string Visit(CreateTableStatement statement) =>
        new CreateTableStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateIndexStatement statement) =>
        new CreateIndexStatementTextRenderer().Render(statement);

    public virtual string Visit(ExecuteStoredProcedureStatement statement) =>
        new ExecuteStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(ReturnStatement statement) =>
        new ReturnStatementTextRenderer().Render(statement);

    public virtual string Visit(SelectInsertedIdStatement statement) =>
        new SelectInsertedIdStatementTextRenderer().Render(statement);

    public virtual string Visit(BulkInsertStatement statement) =>
        new BulkInsertStatementTextRenderer().Render(statement);

    public virtual string Visit(BulkUpdateStatement statement) =>
        new BulkUpdateStatementTextRenderer().Render(statement);

    public virtual string Visit(BulkDeleteStatement statement) =>
        new BulkDeleteStatementTextRenderer().Render(statement);

    public virtual string Visit(DropTableStatement statement) =>
        new DropTableStatementTextRenderer().Render(statement);

    public virtual string Visit(AlterTableStatement statement) =>
        new AlterTableStatementTextRenderer().Render(statement);

    public virtual string Visit(DropIndexStatement statement) =>
        new DropIndexStatementTextRenderer().Render(statement);

    public virtual string Visit(DropTriggerStatement statement) =>
        new DropTriggerStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateTriggerStatement statement) =>
        new CreateTriggerStatementTextRenderer().Render(statement);

    public virtual string Visit(IdentityInsertStatement statement) =>
        new IdentityInsertStatementTextRenderer().Render(statement);

    public virtual string Visit(SqlSeedStatement statement) =>
        throw new InvalidOperationException("SqlSeedStatement is rendered by the SQL driver.");

    public virtual string Visit(ForeignKeyStatement statement) =>
        throw new InvalidOperationException("ForeignKeyStatement is rendered by the SQL driver.");

    public virtual string Visit(DropRoutineStatement statement) =>
        new DropRoutineStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateCreateStoredProcedureStatement statement) =>
        new CreateCreateStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateReadStoredProcedureStatement statement) =>
        new CreateReadStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateNestedReadStoredProcedureStatement statement) =>
        new CreateNestedReadStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateUpdateStoredProcedureStatement statement) =>
        new CreateUpdateStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateDeleteStoredProcedureStatement statement) =>
        new CreateDeleteStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateBulkCreateStoredProcedureStatement statement) =>
        new CreateBulkCreateStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateBulkUpdateStoredProcedureStatement statement) =>
        new CreateBulkUpdateStoredProcedureStatementTextRenderer().Render(statement);

    public virtual string Visit(CreateBulkDeleteStoredProcedureStatement statement) =>
        new CreateBulkDeleteStoredProcedureStatementTextRenderer().Render(statement);

    public virtual IReadOnlyList<IStatement> Procedures(ProcedureSource source) => [];
}
