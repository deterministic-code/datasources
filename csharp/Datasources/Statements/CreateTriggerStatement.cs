namespace Datasources;

public sealed record CreateTriggerStatement(
    string Table,
    string Name,
    string UpdatedColumn,
    string KeyColumn) : ICreateTriggerStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
