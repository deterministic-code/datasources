using System.Globalization;

namespace Datasources;

public class ExpressionRenderer(StatementTextRenderer text)
{
    public virtual string Render(IExpression expression) => expression switch
    {
        Where where => Render(where.Condition),
        Identifier id => id.Name,
        Literal lit => Inline(lit.Value),
        ArrayExpression arr => string.Join(", ", arr.Elements.Select(Render)),
        Operator.Not not => $"NOT ({Render(not.Argument)})",
        Operator.IsNull { Argument: var argument } => $"{Render(argument)} IS NULL",
        Operator.IsNotNull { Argument: var argument } => $"{Render(argument)} IS NOT NULL",
        Operator.Exists { Argument: var argument } => $"EXISTS {Render(argument)}",
        Subquery sub => $"({text.Render(sub.Query)})",
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

    string Wrap(IExpression child, bool and) =>
        NeedsParens(child, and) ? $"({Render(child)})" : Render(child);

    static bool NeedsParens(IExpression child, bool and) => child switch
    {
        Where where => NeedsParens(where.Condition, and),
        Operator.Not or Operator.In or Operator.NotIn => true,
        Operator.And when !and => true,
        Operator.Or when and => true,
        _ => false
    };

    static string Symbol(IExpression op) => op switch
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

    static (IExpression Left, IExpression Right)? Pair(IExpression expression) => expression switch
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

    static string Inline(Value value) => value switch
    {
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Float(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Decimal(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Bool(var flag) => flag ? "true" : "false",
        Value.Text(var text) => $"'{text.Replace("'", "''")}'",
        Value.Null => "null",
        _ => value.ToObject()?.ToString() ?? "null"
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
