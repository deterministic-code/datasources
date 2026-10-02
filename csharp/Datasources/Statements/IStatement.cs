namespace Datasources;

public interface IStatement
{
    string Accept(IStatementVisitor visitor);
}
