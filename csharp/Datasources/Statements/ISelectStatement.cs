namespace Datasources;

public interface ISelectStatement : IStatement
{
    IWhere? Where { get; }
}
