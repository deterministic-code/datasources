namespace Datasources;

public sealed record CreateRoutineStatement(
    string Name,
    RoutineKind Kind,
    string Table,
    IReadOnlyList<string> Columns,
    IReadOnlyList<RoutineParameter> Parameters,
    string Key,
    string KeyType,
    IReadOnlyList<string> Writable,
    string? ByField = null,
    bool GeneratedKey = false,
    string? AuditUpdated = null) : ICreateRoutineStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
