namespace Datasources;

public sealed record QueryStatement : IQuery
{
    readonly string text;
    public QueryStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public QueryStatement(QueryStatementOptions options) : this(Select(options), options.Where) { }
    public IWhere? Where { get; }
    public string GetText() => text;

    static string Select(QueryStatementOptions options) =>
        $"{WithWhere($"SELECT {(options.Fields.Count == 0 ? "*" : string.Join(", ", options.Fields))} FROM {options.Table}{JoinSql(options.Joins)}", options.Where)}{GroupSql(options.GroupBy)}{OrderSql(options.OrderBy)}{LimitSql(options.Limit)}";

    static string GroupSql(IReadOnlyList<string>? columns) =>
        columns is null or [] ? "" : $" GROUP BY {string.Join(", ", columns)}";

    static string OrderSql(IReadOnlyList<OrderBy>? order) =>
        order is null or [] ? "" : $" ORDER BY {string.Join(", ", order)}";

    static string LimitSql(int? limit) => limit switch
    {
        null => "",
        < 0 => throw new InvalidOperationException("negative limit"),
        var n => $" LIMIT {n}"
    };

    static string WithWhere(string sql, IWhere? where) =>
        where is null ? sql : $"{sql} WHERE {where.GetText()}";

    static string JoinSql(IReadOnlyList<Join>? joins) =>
        joins is null or []
            ? ""
            : string.Concat(joins.Select(join => $" {Keyword(join.Kind)} {join.Table} ON {join.LeftColumn} = {join.RightColumn}"));

    static string Keyword(JoinKind kind) => kind switch
    {
        JoinKind.Inner => "INNER JOIN",
        JoinKind.Left => "LEFT JOIN",
        JoinKind.Right => "RIGHT JOIN",
        _ => throw new InvalidOperationException("empty join")
    };
}
