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
}
