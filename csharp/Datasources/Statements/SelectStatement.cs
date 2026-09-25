namespace Datasources;

public sealed record SelectStatement : ISelectStatement
{
    readonly string text;

    public SelectStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);

    public SelectStatement(SelectStatementOptions options) : this(Select(options), options.Where)
    {
        Distinct = options.Distinct;
        Table = options.Table;
        Fields = options.Fields;
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
    public IReadOnlyList<Join> Joins { get; } = [];
    public IWhere? Where { get; }
    public IReadOnlyList<string> GroupBy { get; } = [];
    public IWhere? Having { get; }
    public IReadOnlyList<OrderBy> OrderBy { get; } = [];
    public int? Limit { get; }
    public int? Offset { get; }
    public string GetText() => text;

    static string Select(SelectStatementOptions options) =>
        $"SELECT {(options.Distinct ? "DISTINCT " : "")}{(options.Fields.Count == 0 ? "*" : string.Join(", ", options.Fields))} FROM {options.Table}{JoinSql(options.Joins)}{WhereSql(options.Where)}{GroupSql(options.GroupBy)}{HavingSql(options.Having)}{OrderSql(options.OrderBy)}{LimitSql(options.Limit)}{OffsetSql(options.Offset)}";

    static string WhereSql(IWhere? where) => where is null ? "" : $" WHERE {where.GetText()}";

    static string HavingSql(IWhere? having) => having is null ? "" : $" HAVING {having.GetText()}";

    static string GroupSql(IReadOnlyList<string>? columns) =>
        columns is null or [] ? "" : $" GROUP BY {string.Join(", ", columns)}";

    static string OrderSql(IReadOnlyList<OrderBy>? order) =>
        order is null or [] ? "" : $" ORDER BY {string.Join(", ", order)}";

    static string LimitSql(int? limit) => Bound("LIMIT", limit);

    static string OffsetSql(int? offset) => Bound("OFFSET", offset);

    static string Bound(string keyword, int? value) => value switch
    {
        null => "",
        < 0 => throw new InvalidOperationException($"negative {keyword.ToLowerInvariant()}"),
        var n => $" {keyword} {n}"
    };

    static string JoinSql(IReadOnlyList<Join>? joins) =>
        joins is null or []
            ? ""
            : string.Concat(joins.Select(join => join.Kind == JoinKind.Cross
                ? $" CROSS JOIN {join.Table}"
                : $" {Keyword(join.Kind)} {join.Table} ON {new Where(join.On ?? throw new InvalidOperationException("empty join")).GetText()}"));

    static string Keyword(JoinKind kind) => kind switch
    {
        JoinKind.Inner => "INNER JOIN",
        JoinKind.Left => "LEFT JOIN",
        JoinKind.Right => "RIGHT JOIN",
        JoinKind.Full => "FULL JOIN",
        JoinKind.Cross => "CROSS JOIN",
        _ => throw new InvalidOperationException("empty join")
    };
}
