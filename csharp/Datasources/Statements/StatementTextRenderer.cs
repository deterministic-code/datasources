namespace Datasources;

public class StatementTextRenderer
{
    protected IStatementVisitor Statements { get; }
    protected IConditionVisitor Conditions { get; }

    public StatementTextRenderer(
        Func<StatementTextRenderer, StatementRenderer>? statements = null,
        Func<ExpressionRenderer, ConditionRenderer>? conditions = null,
        Func<StatementTextRenderer, ExpressionRenderer>? expressions = null)
    {
        var expressionRenderer = expressions?.Invoke(this) ?? new ExpressionRenderer(this);
        Statements = statements?.Invoke(this) ?? new StatementRenderer(this);
        Conditions = conditions?.Invoke(expressionRenderer) ?? new ConditionRenderer(expressionRenderer);
    }

    public string Render(IStatement statement) => statement switch
    {
        SelectStatement s => s.Accept(Statements),
        InsertStatement s => s.Accept(Statements),
        UpdateStatement s => s.Accept(Statements),
        DeleteStatement s => s.Accept(Statements),
        CreateTableStatement s => s.Accept(Statements),
        CreateIndexStatement s => s.Accept(Statements),
        ExecuteStoredProcedureStatement s => s.Accept(Statements),
        ReturnStatement s => s.Accept(Statements),
        SelectInsertedIdStatement s => s.Accept(Statements),
        BulkInsertStatement s => s.Accept(Statements),
        BulkUpdateStatement s => s.Accept(Statements),
        BulkDeleteStatement s => s.Accept(Statements),
        DropTableStatement s => s.Accept(Statements),
        AlterTableStatement s => s.Accept(Statements),
        DropIndexStatement s => s.Accept(Statements),
        DropTriggerStatement s => s.Accept(Statements),
        CreateTriggerStatement s => s.Accept(Statements),
        IdentityInsertStatement s => s.Accept(Statements),
        DropRoutineStatement s => s.Accept(Statements),
        CreateCreateStoredProcedureStatement s => s.Accept(Statements),
        CreateReadStoredProcedureStatement s => s.Accept(Statements),
        CreateUpdateStoredProcedureStatement s => s.Accept(Statements),
        CreateDeleteStoredProcedureStatement s => s.Accept(Statements),
        CreateBulkCreateStoredProcedureStatement s => s.Accept(Statements),
        CreateBulkUpdateStoredProcedureStatement s => s.Accept(Statements),
        CreateBulkDeleteStoredProcedureStatement s => s.Accept(Statements),
        _ => statement.Accept(Statements)
    };

    public string Render(ICondition condition) => condition switch
    {
        Where w => w.Accept(Conditions),
        WhereText w => w.Accept(Conditions),
        _ => condition.Accept(Conditions)
    };
}
