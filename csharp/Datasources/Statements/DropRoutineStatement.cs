namespace Datasources;

public sealed record DropRoutineStatement(string Name) : IDropRoutineStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
