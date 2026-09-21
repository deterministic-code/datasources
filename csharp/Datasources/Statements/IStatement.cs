namespace Datasources;

public interface IStatement : IText
{
    IReadOnlyList<Parameter> Parameters => [];
}
