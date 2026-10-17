namespace Datasources;

public sealed record CreateUpdateStoredProcedureStatement(
    string Name,
    string Table,
    IReadOnlyList<string> Columns,
    IReadOnlyList<RoutineParameter> Parameters,
    string Key,
    string KeyType,
    IReadOnlyList<string> Writable,
    IReadOnlyList<string> ByProperties,
    bool GeneratedKey = false,
    string? AuditUpdated = null) : ICreateUpdateStoredProcedureStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
