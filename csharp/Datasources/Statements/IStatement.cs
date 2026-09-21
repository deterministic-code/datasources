namespace Datasources;

public interface IStatement : IExpression
{
    string GetText();
    IReadOnlyList<Parameter> Parameters => [];
}
