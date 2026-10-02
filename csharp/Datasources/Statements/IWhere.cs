namespace Datasources;

public interface IWhere : IExpression, ICondition
{
    IReadOnlyList<Parameter> Parameters => [];
}
