namespace Datasources;

public sealed record CreateCreateStoredProcedureStatement(
    string Name,
    string Table,
    IReadOnlyList<string> Columns,
    IReadOnlyList<RoutineParameter> Parameters,
    string Key,
    string KeyType,
    IReadOnlyList<string> Writable,
    bool GeneratedKey = false,
    string? AuditUpdated = null) : ICreateCreateStoredProcedureStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
