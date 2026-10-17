namespace Datasources;

public sealed record CreateBulkCreateStoredProcedureStatement(
    string Name,
    string Table,
    IReadOnlyList<RoutineParameter> Columns,
    RoutineParameter Payload,
    string Key,
    string KeyType) : ICreateBulkCreateStoredProcedureStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
