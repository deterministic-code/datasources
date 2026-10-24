namespace Datasources;

public sealed class SelectStatementOptions
{
    public string Table { get; init; } = "";
    public IReadOnlyList<string> Fields { get; init; } = [];
    public IReadOnlyList<string?> Aliases { get; init; } = [];
    public bool Distinct { get; init; }
    public IWhere? Where { get; init; }
    public IReadOnlyList<Join>? Joins { get; init; }
    public IReadOnlyList<string>? GroupBy { get; init; }
    public IWhere? Having { get; init; }
    public IReadOnlyList<OrderBy>? OrderBy { get; init; }
    public int? Limit { get; init; }
    public int? Offset { get; init; }
}
