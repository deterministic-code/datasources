namespace Datasources;

public class SelectStatementTextRenderer(StatementTextRenderer text)
{
    public virtual string Render(SelectStatement statement) =>
        $"SELECT {(statement.Distinct ? "DISTINCT " : "")}{FieldsSql(statement)} FROM {statement.Table}{JoinSql(statement.Joins)}{WhereSql(statement.Where)}{GroupSql(statement.GroupBy)}{HavingSql(statement.Having)}{OrderSql(statement.OrderBy)}{Limit(statement.Limit)}{Offset(statement.Offset)}";

    static string FieldsSql(SelectStatement statement) =>
        statement.Fields.Count == 0
            ? "*"
            : string.Join(", ", statement.Fields.Select((field, i) =>
                SelectFieldAliases.Apply(field, i < statement.Aliases.Count ? statement.Aliases[i] : null)));

    protected virtual string WhereSql(IWhere? where) =>
        where is null ? "" : $" WHERE {text.Render(where)}";

    protected virtual string HavingSql(IWhere? having) =>
        having is null ? "" : $" HAVING {text.Render(having)}";

    protected virtual string GroupSql(IReadOnlyList<string>? columns) =>
        columns is null or [] ? "" : $" GROUP BY {string.Join(", ", columns)}";

    protected virtual string OrderSql(IReadOnlyList<OrderBy>? order) =>
        order is null or [] ? "" : $" ORDER BY {string.Join(", ", order)}";

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
                ? $" CROSS JOIN {join.Table}"
                : $" {Keyword(join.Kind)} {join.Table} ON {text.Render(new Where(join.On ?? throw new InvalidOperationException("empty join")))}"));

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
