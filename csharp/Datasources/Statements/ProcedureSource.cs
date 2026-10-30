namespace Datasources;

public sealed record ProcedureByGroup(
    IReadOnlyList<string> Logical,
    IReadOnlyList<string> Columns,
    IReadOnlyList<RoutineParameter> Parameters,
    IReadOnlyList<RoutineParameter> UpdateParameters);

public sealed class ProcedureSource
{
    public required string Plural { get; init; }
    public required string Table { get; init; }
    public required IReadOnlyList<string> Columns { get; init; }
    public required IReadOnlyList<RoutineParameter> CreateParameters { get; init; }
    public required IReadOnlyList<string> Writable { get; init; }
    public required IReadOnlyList<RoutineParameter> CreateColumns { get; init; }
    public required IReadOnlyList<RoutineParameter> UpdateColumns { get; init; }
    public required IReadOnlyList<RoutineParameter> TypedColumns { get; init; }
    public required IReadOnlyList<ProcedureByGroup> ByGroups { get; init; }
    public required string Key { get; init; }
    public required string KeyType { get; init; }
    public string? AuditUpdated { get; init; }
    public bool GeneratedKey { get; init; }
    public required Func<string, string> RoutineName { get; init; }
}
