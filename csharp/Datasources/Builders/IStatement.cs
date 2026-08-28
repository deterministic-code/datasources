namespace Datasources;

public interface IStatement
{
    string GetText();
    IReadOnlyList<Parameter> Parameters => [];
}
