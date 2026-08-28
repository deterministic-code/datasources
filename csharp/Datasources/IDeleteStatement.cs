namespace Datasources;

public interface IDeleteStatement : IStatement
{
    IWhere? Where { get; }
}
