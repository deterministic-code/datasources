namespace Datasources;

public interface IQuery : IStatement
{
    IWhere? Where { get; }
}
