namespace Datasources;

public interface IWhere : IExpression, IText
{
    IReadOnlyList<Parameter> Parameters => [];
}
