namespace Datasources;

public sealed record QueryStatement : IQuery
{
    readonly string text;
    public QueryStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public QueryStatement(
        string table,
        IReadOnlyList<string> fields,
        IWhere? where = null,
        IReadOnlyList<Join>? joins = null)
        : this(Select(table, fields, joins, where), where) { }
    public IWhere? Where { get; }
    public string GetText() => text;

    static string Select(string table, IReadOnlyList<string> fields, IReadOnlyList<Join>? joins, IWhere? where) =>
        WithWhere($"SELECT {(fields.Count == 0 ? "*" : string.Join(", ", fields))} FROM {table}{JoinSql(joins)}", where);

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
