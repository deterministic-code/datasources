namespace Datasources;

public class ConditionRenderer(ExpressionRenderer expressions) : IConditionVisitor
{
    public virtual string Render(ICondition condition) => condition switch
    {
        Where w => w.Accept(this),
        WhereText w => w.Accept(this),
        _ => condition.Accept(this)
    };

    public virtual string Visit(Where where) => expressions.Render(where.Condition);

    public virtual string Visit(WhereText where) => where.RawText;
}
