namespace Datasources;

public interface IUpdateStatement : IStatement
{
    IWhere? Where { get; }
}
