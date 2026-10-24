namespace Datasources;

public sealed record SelectStatement : ISelectStatement
{
    public SelectStatement(SelectStatementOptions options)
    {
        Distinct = options.Distinct;
        Table = options.Table;
        Fields = options.Fields;
        Aliases = options.Aliases;
        Where = options.Where;
        Joins = options.Joins ?? [];
        GroupBy = options.GroupBy ?? [];
        Having = options.Having;
        OrderBy = options.OrderBy ?? [];
        Limit = options.Limit;
        Offset = options.Offset;
    }

    public bool Distinct { get; }
    public string? Table { get; }
    public IReadOnlyList<string> Fields { get; } = [];
    public IReadOnlyList<string?> Aliases { get; } = [];
    public IReadOnlyList<Join> Joins { get; } = [];
    public IWhere? Where { get; }
    public IReadOnlyList<string> GroupBy { get; } = [];
    public IWhere? Having { get; }
    public IReadOnlyList<OrderBy> OrderBy { get; } = [];
    public int? Limit { get; }
    public int? Offset { get; }

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
