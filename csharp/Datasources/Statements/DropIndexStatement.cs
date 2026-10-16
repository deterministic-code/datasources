namespace Datasources;

public sealed record DropIndexStatement(string Name, string Table) : IDropIndexStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
