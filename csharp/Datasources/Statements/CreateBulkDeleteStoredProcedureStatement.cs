namespace Datasources;

public sealed record CreateBulkDeleteStoredProcedureStatement(
    string Name,
    string Table,
    IReadOnlyList<RoutineParameter> Columns,
    RoutineParameter Payload,
    string Key,
    string KeyType) : ICreateBulkDeleteStoredProcedureStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
