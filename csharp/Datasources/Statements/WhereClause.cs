using System.Globalization;

namespace Datasources;

public sealed record WhereClause(IExpression Condition) : IWhere
{
    public string GetText() => Render(Condition);
    public IReadOnlyList<Parameter> Parameters => Collect(Condition);

    private static string Render(IExpression expression) => expression switch
    {
        WhereClause where => Render(where.Condition),
        Identifier id => id.Name,
        Literal lit => Inline(lit.Value),
        ArrayExpression arr => string.Join(", ", arr.Elements.Select(Render)),
        Operator.Not not => $"NOT ({Render(not.Argument)})",
        Operator.In { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Count == 0
                ? "1 = 0"
                : $"{id.Name} IN ({string.Join(", ", arr.Elements.Select((_, i) => $"${id.Name}_{i}"))})",
        Operator.And(var left, var right) =>
            $"{Wrap(left, and: true)} AND {Wrap(right, and: true)}",
        Operator.Or(var left, var right) =>
            $"{Wrap(left, and: false)} OR {Wrap(right, and: false)}",
        { } op when Pair(op) is ({ } left, { } right) && left is Identifier id && right is Literal =>
            $"{id.Name} {Symbol(op)} ${id.Name}",
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
        WhereClause where => NeedsParens(where.Condition, and),
        Operator.Not or Operator.In => true,
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
        _ => null
    };

    private static string Inline(Value value) => value switch
    {
        Value.Int(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Float(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Decimal(var number) => number.ToString(CultureInfo.InvariantCulture),
        Value.Bool(var flag) => flag ? "true" : "false",
        Value.Text(var text) => text,
        Value.Null => "null",
        _ => value.ToObject()?.ToString() ?? "null"
    };

    private static IReadOnlyList<Parameter> Collect(IExpression expression) => expression switch
    {
        WhereClause where => Collect(where.Condition),
        Literal or Identifier => [],
        Operator.Not not => Collect(not.Argument),
        Operator.In { Left: Identifier id, Right: ArrayExpression arr } =>
            arr.Elements.Select((element, i) => element is Literal lit
                ? new Parameter($"{id.Name}_{i}", lit.Value)
                : throw new InvalidOperationException("empty in")).ToArray(),
        { } op when Pair(op) is (Identifier id, Literal lit) => [new Parameter(id.Name, lit.Value)],
        { } op when Pair(op) is (Literal, Literal) => [],
        { } op when Pair(op) is ({ } left, { } right) => [..Collect(left), ..Collect(right)],
        ArrayExpression arr => arr.Elements.SelectMany(Collect).ToArray(),
        _ => throw new InvalidOperationException("empty expression")
    };
}
