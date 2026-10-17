namespace Datasources;

public sealed record CreateBulkUpdateStoredProcedureStatement(
    string Name,
    string Table,
    IReadOnlyList<RoutineParameter> Columns,
    RoutineParameter Payload,
    string Key,
    string KeyType) : ICreateBulkUpdateStoredProcedureStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
