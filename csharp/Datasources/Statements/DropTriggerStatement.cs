namespace Datasources;

public sealed record DropTriggerStatement(string Name, string Table) : IDropTriggerStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
