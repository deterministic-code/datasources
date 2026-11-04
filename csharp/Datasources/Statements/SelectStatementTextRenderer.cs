namespace Datasources;

public class SelectStatementTextRenderer(StatementTextRenderer text)
{
    public virtual string Render(SelectStatement statement) =>
        $"SELECT {(statement.Distinct ? "DISTINCT " : "")}{FieldsSql(statement)} FROM {Quote(statement.Table ?? "")}{JoinSql(statement.Joins)}{WhereSql(statement.Where)}{GroupSql(statement.GroupBy)}{HavingSql(statement.Having)}{OrderSql(statement.OrderBy)}{Limit(statement.Limit)}{Offset(statement.Offset)}";

    protected virtual string Quote(string name) => name;

    string QuotePath(string name) =>
        name.Contains('(') ? name : string.Join(".", SelectFieldAliases.Segments(name).Select(Quote));

    string FieldsSql(SelectStatement statement) =>
        statement.Fields.Count == 0
            ? "*"
            : string.Join(", ", statement.Fields.Select((field, i) =>
                FieldSql(field, i < statement.Aliases.Count ? statement.Aliases[i] : null)));

    string FieldSql(string field, string? alias)
    {
        var rendered = SelectFieldAliases.AsIndex(field) >= 0 || field.Contains('(') ? field : QuotePath(field);
        return alias is { } name ? $"{rendered} AS {Quote(name)}" : rendered;
    }

    protected virtual string WhereSql(IWhere? where) =>
        where is null ? "" : $" WHERE {text.Render(where)}";

    protected virtual string HavingSql(IWhere? having) =>
        having is null ? "" : $" HAVING {text.Render(having)}";

    protected virtual string GroupSql(IReadOnlyList<string>? columns) =>
        columns is null or [] ? "" : $" GROUP BY {string.Join(", ", columns.Select(QuotePath))}";

    protected virtual string OrderSql(IReadOnlyList<OrderBy>? order) =>
        order is null or [] ? "" : $" ORDER BY {string.Join(", ", order.Select(item => $"{QuotePath(item.Column)} {(item.Direction == SortDirection.Desc ? "DESC" : "ASC")}"))}";

    protected virtual string Limit(int? limit) => Bound("LIMIT", limit);

    protected virtual string Offset(int? offset) => Bound("OFFSET", offset);

    protected virtual string Bound(string keyword, int? value) => value switch
    {
        null => "",
        < 0 => throw new InvalidOperationException($"negative {keyword.ToLowerInvariant()}"),
        var n => $" {keyword} {n}"
    };

    protected virtual string JoinSql(IReadOnlyList<Join>? joins) =>
        joins is null or []
            ? ""
            : string.Concat(joins.Select(join => join.Kind == JoinKind.Cross
                ? $" CROSS JOIN {Quote(join.Table)}"
                : $" {Keyword(join.Kind)} {Quote(join.Table)} ON {text.Render(new Where(join.On ?? throw new InvalidOperationException("empty join")))}"));

    protected virtual string Keyword(JoinKind kind) => kind switch
    {
        JoinKind.Inner => "INNER JOIN",
        JoinKind.Left => "LEFT JOIN",
        JoinKind.Right => "RIGHT JOIN",
        JoinKind.Full => "FULL JOIN",
        JoinKind.Cross => "CROSS JOIN",
        _ => throw new InvalidOperationException("empty join")
    };
}
