using System.Globalization;

namespace Datasources;

public sealed record Where(IExpression Condition) : IWhere
{
    public Where(string column, Comparison op, Value? value)
        : this(op(column, value)) { }

    public Where And(string column, Comparison op, Value? value) =>
        new(new Operator.And(Condition, op(column, value)));

    public Where Or(string column, Comparison op, Value? value) =>
        new(new Operator.Or(Condition, op(column, value)));

    public Where Map(Func<string, string> column, Func<string, Value, Value> value) =>
        new(Rewrite(Condition, column, value));

    public string GetText() => Render(Condition);
    public IReadOnlyList<Parameter> Parameters => Collect(Condition);

    private static string Render(IExpression expression) => expression switch
    {
        Where where => Render(where.Condition),
        Identifier id => id.Name,
        Literal lit => Inline(lit.Value),
        ArrayExpression arr => string.Join(", ", arr.Elements.Select(Render)),
        Operator.Not not => $"NOT ({Render(not.Argument)})",
        Operator.IsNull { Argument: var argument } => $"{Render(argument)} IS NULL",
        Operator.IsNotNull { Argument: var argument } => $"{Render(argument)} IS NOT NULL",
        Operator.Exists { Argument: var argument } => $"EXISTS {Render(argument)}",
        Subquery sub => $"({sub.Query.GetText()})",
        Operator.In { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Count == 0
                ? "1 = 0"
                : $"{id.Name} IN ({string.Join(", ", arr.Elements.Select((_, i) => $"${ParameterName(id.Name)}_{i}"))})",
        Operator.In { Left: Identifier id, Right: Subquery } inn => $"{id.Name} IN {Render(inn.Right)}",
        Operator.NotIn { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Count == 0
                ? "1 = 1"
                : $"{id.Name} NOT IN ({string.Join(", ", arr.Elements.Select((_, i) => $"${ParameterName(id.Name)}_{i}"))})",
        Operator.NotIn { Left: Identifier id, Right: Subquery } miss => $"{id.Name} NOT IN {Render(miss.Right)}",
        Operator.Between { Value: Identifier id, Low: Literal, High: Literal } =>
            $"{id.Name} BETWEEN ${ParameterName(id.Name)}_0 AND ${ParameterName(id.Name)}_1",
        Operator.Between(var value, var low, var high) =>
            $"{Render(value)} BETWEEN {Render(low)} AND {Render(high)}",
        Operator.NotBetween { Value: Identifier id, Low: Literal, High: Literal } =>
            $"{id.Name} NOT BETWEEN ${ParameterName(id.Name)}_0 AND ${ParameterName(id.Name)}_1",
        Operator.NotBetween(var value, var low, var high) =>
            $"{Render(value)} NOT BETWEEN {Render(low)} AND {Render(high)}",
        Operator.And(var left, var right) =>
            $"{Wrap(left, and: true)} AND {Wrap(right, and: true)}",
        Operator.Or(var left, var right) =>
            $"{Wrap(left, and: false)} OR {Wrap(right, and: false)}",
        { } op when Pair(op) is ({ } left, { } right) && left is Identifier id && right is Literal =>
            $"{id.Name} {Symbol(op)} ${ParameterName(id.Name)}",
        { } op when Pair(op) is ({ } left, { } right) && left is Literal a && right is Literal b =>
            $"{Inline(a.Value)} {Symbol(op)} {Inline(b.Value)}",
        { } op when Pair(op) is ({ } left, { } right) =>
            $"{Render(left)} {Symbol(op)} {Render(right)}",
        _ => throw new InvalidOperationException("empty expression")
    };

    private static string Wrap(IExpression child, bool and) =>
        NeedsParens(child, and) ? $"({Render(child)})" : Render(child);

    private static bool NeedsParens(IExpression child, bool and) => child switch
    {
        Where where => NeedsParens(where.Condition, and),
        Operator.Not or Operator.In or Operator.NotIn => true,
        Operator.And when !and => true,
        Operator.Or when and => true,
        _ => false
    };

    private static string Symbol(IExpression op) => op switch
    {
        Operator.Equals => "=",
        Operator.NotEquals => "<>",
        Operator.GreaterThan => ">",
        Operator.LessThan => "<",
        Operator.GreaterThanOrEqual => ">=",
        Operator.LessThanOrEqual => "<=",
        Operator.Like => "LIKE",
        Operator.NotLike => "NOT LIKE",
        _ => "IN"
    };

    private static (IExpression Left, IExpression Right)? Pair(IExpression expression) => expression switch
    {
        Operator.Equals(var left, var right) => (left, right),
        Operator.NotEquals(var left, var right) => (left, right),
        Operator.GreaterThan(var left, var right) => (left, right),
        Operator.LessThan(var left, var right) => (left, right),
        Operator.GreaterThanOrEqual(var left, var right) => (left, right),
        Operator.LessThanOrEqual(var left, var right) => (left, right),
        Operator.And(var left, var right) => (left, right),
        Operator.Or(var left, var right) => (left, right),
        Operator.In(var left, var right) => (left, right),
        Operator.Like(var left, var right) => (left, right),
        Operator.NotLike(var left, var right) => (left, right),
        _ => null
    };

    private static string Inline(Value value) => value switch
    {
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Float(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Decimal(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Bool(var flag) => flag ? "true" : "false",
        Value.Text(var text) => $"'{text.Replace("'", "''")}'",
        Value.Null => "null",
        _ => value.ToObject()?.ToString() ?? "null"
    };

    private static IReadOnlyList<Parameter> Collect(IExpression expression) => expression switch
    {
        Where where => Collect(where.Condition),
        Literal or Identifier => [],
        Operator.Not not => Collect(not.Argument),
        Operator.IsNull or Operator.IsNotNull => [],
        Operator.Exists { Argument: var argument } => Collect(argument),
        Subquery sub => [
            ..sub.Query.Where?.Parameters ?? [],
            ..sub.Query.Having?.Parameters ?? [],
            ..sub.Query.Joins.SelectMany(join => join.On is null ? [] : Collect(join.On))],
        Operator.In { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Select((element, i) => element is Literal lit
                ? new Parameter($"{ParameterName(id.Name)}_{i}", lit.Value)
                : throw new InvalidOperationException("empty in")).ToArray(),
        Operator.NotIn { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Select((element, i) => element is Literal lit
                ? new Parameter($"{ParameterName(id.Name)}_{i}", lit.Value)
                : throw new InvalidOperationException("empty in")).ToArray(),
        Operator.Between { Value: Identifier id, Low: Literal low, High: Literal high } =>
            [new Parameter($"{ParameterName(id.Name)}_0", low.Value), new Parameter($"{ParameterName(id.Name)}_1", high.Value)],
        Operator.NotBetween { Value: Identifier id, Low: Literal low, High: Literal high } =>
            [new Parameter($"{ParameterName(id.Name)}_0", low.Value), new Parameter($"{ParameterName(id.Name)}_1", high.Value)],
        Operator.Between(var value, var low, var high) => [..Collect(value), ..Collect(low), ..Collect(high)],
        Operator.NotBetween(var value, var low, var high) => [..Collect(value), ..Collect(low), ..Collect(high)],
        Operator.NotIn(var left, var right) => [..Collect(left), ..Collect(right)],
        { } op when Pair(op) is (Identifier id, Literal lit) => [new Parameter(ParameterName(id.Name), lit.Value)],
        { } op when Pair(op) is (Literal, Literal) => [],
        { } op when Pair(op) is ({ } left, { } right) => [..Collect(left), ..Collect(right)],
        ArrayExpression arr => arr.Elements.SelectMany(Collect).ToArray(),
        _ => throw new InvalidOperationException("empty expression")
    };

    static IExpression Rewrite(
        IExpression expression, Func<string, string> column, Func<string, Value, Value> value) =>
        expression switch
        {
            Where where => Rewrite(where.Condition, column, value),
            Identifier id => new Identifier(column(id.Name)),
            Literal => expression,
            ArrayExpression arr => new ArrayExpression([.. arr.Elements.Select(element => Rewrite(element, column, value))]),
            Operator.Not not => new Operator.Not(Rewrite(not.Argument, column, value)),
            Operator.IsNull { Argument: var argument } => new Operator.IsNull(Rewrite(argument, column, value)),
            Operator.IsNotNull { Argument: var argument } => new Operator.IsNotNull(Rewrite(argument, column, value)),
            Operator.Exists { Argument: var argument } => new Operator.Exists(Rewrite(argument, column, value)),
            Subquery sub => new Subquery(RewriteQuery(sub.Query, column, value)),
            Operator.And(var left, var right) => new Operator.And(Rewrite(left, column, value), Rewrite(right, column, value)),
            Operator.Or(var left, var right) => new Operator.Or(Rewrite(left, column, value), Rewrite(right, column, value)),
            Operator.In(var left, var right) => new Operator.In(Rewrite(left, column, value), InRight(left, right, column, value)),
            Operator.NotIn(var left, var right) => new Operator.NotIn(Rewrite(left, column, value), InRight(left, right, column, value)),
            Operator.Like(var left, var right) => new Operator.Like(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.NotLike(var left, var right) => new Operator.NotLike(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.Between(var subject, var low, var high) => new Operator.Between(Rewrite(subject, column, value), Bound(subject, low, column, value), Bound(subject, high, column, value)),
            Operator.NotBetween(var subject, var low, var high) => new Operator.NotBetween(Rewrite(subject, column, value), Bound(subject, low, column, value), Bound(subject, high, column, value)),
            Operator.Equals(var left, var right) => new Operator.Equals(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.NotEquals(var left, var right) => new Operator.NotEquals(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.GreaterThan(var left, var right) => new Operator.GreaterThan(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.LessThan(var left, var right) => new Operator.LessThan(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.GreaterThanOrEqual(var left, var right) => new Operator.GreaterThanOrEqual(Side(left, right, column, value), Side(right, left, column, value)),
            Operator.LessThanOrEqual(var left, var right) => new Operator.LessThanOrEqual(Side(left, right, column, value), Side(right, left, column, value)),
            _ => throw new InvalidOperationException("empty expression")
        };

    static IExpression Side(
        IExpression expression, IExpression other, Func<string, string> column, Func<string, Value, Value> value) =>
        expression is Literal lit && other is Identifier id
            ? new Literal(value(id.Name, lit.Value))
            : Rewrite(expression, column, value);

    static IExpression InRight(
        IExpression left, IExpression right, Func<string, string> column, Func<string, Value, Value> value) =>
        left is Identifier id && right is ArrayExpression arr
            ? new ArrayExpression([.. arr.Elements.Select(element => element is Literal lit
                ? (IExpression)new Literal(value(id.Name, lit.Value))
                : Rewrite(element, column, value))])
            : Rewrite(right, column, value);

    static IExpression Bound(
        IExpression subject, IExpression bound, Func<string, string> column, Func<string, Value, Value> value) =>
        subject is Identifier id && bound is Literal lit
            ? new Literal(value(id.Name, lit.Value))
            : Rewrite(bound, column, value);

    static SelectStatement RewriteQuery(
        SelectStatement query, Func<string, string> column, Func<string, Value, Value> value) =>
        query.Table is null
            ? query
            : new SelectStatement(new SelectStatementOptions
            {
                Distinct = query.Distinct,
                Table = query.Table,
                Fields = query.Fields,
                Where = RewriteWhere(query.Where, column, value),
                Joins = query.Joins.Select(join => join.On is null
                    ? join
                    : join with { On = Rewrite(join.On, column, value) }).ToArray(),
                GroupBy = query.GroupBy,
                Having = RewriteWhere(query.Having, column, value),
                OrderBy = query.OrderBy,
                Limit = query.Limit,
                Offset = query.Offset
            });

    static IWhere? RewriteWhere(
        IWhere? where, Func<string, string> column, Func<string, Value, Value> value) => where switch
    {
        null => null,
        Where clause => new Where(Rewrite(clause.Condition, column, value)),
        _ => where
    };

    static string ParameterName(string name) =>
        string.Join(".", Parts(name).Select(part =>
            part.Length >= 2 && part[0] == '"' && part[^1] == '"'
                ? part[1..^1].Replace("\"\"", "\"")
                : part));

    static IEnumerable<string> Parts(string name)
    {
        var start = 0;
        var quoted = false;
        for (var i = 0; i < name.Length; i++)
        {
            if (name[i] == '"') quoted = !quoted;
            if (name[i] != '.' || quoted) continue;
            yield return name[start..i];
            start = i + 1;
        }
        yield return name[start..];
    }
}
