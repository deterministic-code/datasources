namespace Datasources;

public sealed record DropTableStatement(string Table) : IDropTableStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
