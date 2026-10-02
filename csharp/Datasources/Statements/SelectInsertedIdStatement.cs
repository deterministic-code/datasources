namespace Datasources;

public sealed record SelectInsertedIdStatement(string Table, string Column) : IStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
