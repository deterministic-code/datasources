namespace Datasources;

public sealed class QueryStatementOptions
{
    public required string Table { get; init; }
    public required IReadOnlyList<string> Fields { get; init; }
    public IWhere? Where { get; init; }
    public IReadOnlyList<Join>? Joins { get; init; }
    public IReadOnlyList<string>? GroupBy { get; init; }
    public IReadOnlyList<OrderBy>? OrderBy { get; init; }
    public int? Limit { get; init; }
}
