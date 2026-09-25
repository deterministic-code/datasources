namespace Datasources;

public sealed class StatementEvaluator
{
    public Value Evaluate(IExpression expression, IReadOnlyDictionary<string, Value>? parameters) =>
        expression switch
        {
            Where where => Evaluate(where.Condition, parameters),
            WhereText => throw new InvalidOperationException("text where is not evaluable"),
            Identifier id => parameters is not null && parameters.TryGetValue(id.Name, out var value)
                ? value : Value.Null.Instance,
            Literal lit => lit.Value,
            Operator.Not not => new Value.Bool(!IsTrue(Evaluate(not.Argument, parameters))),
            Operator.IsNull { Argument: var argument } => new Value.Bool(Evaluate(argument, parameters) is Value.Null),
            Operator.IsNotNull { Argument: var argument } => new Value.Bool(Evaluate(argument, parameters) is not Value.Null),
            Operator.Like(var left, var right) => new Value.Bool(Like(Evaluate(left, parameters), Evaluate(right, parameters))),
            Operator.NotLike(var left, var right) => new Value.Bool(!Like(Evaluate(left, parameters), Evaluate(right, parameters))),
            Operator.Between(var value, var low, var high) => new Value.Bool(Between(
                Evaluate(value, parameters), Evaluate(low, parameters), Evaluate(high, parameters))),
            Operator.NotBetween(var value, var low, var high) => new Value.Bool(!Between(
                Evaluate(value, parameters), Evaluate(low, parameters), Evaluate(high, parameters))),
            Operator.NotIn(var left, var right) => new Value.Bool(!Contains(Evaluate(left, parameters), right, parameters)),
            Operator.Exists or Subquery => throw new InvalidOperationException("subquery is not evaluable"),
            Operator.And(var left, var right) => new Value.Bool(
                IsTrue(Evaluate(left, parameters)) && IsTrue(Evaluate(right, parameters))),
            Operator.Or(var left, var right) => new Value.Bool(
                IsTrue(Evaluate(left, parameters)) || IsTrue(Evaluate(right, parameters))),
            Operator.In(var left, var right) =>
                new Value.Bool(Contains(Evaluate(left, parameters), right, parameters)),
            Operator.Equals(var left, var right) => new Value.Bool(
                Evaluate(left, parameters).Equals(Evaluate(right, parameters))),
            Operator.NotEquals(var left, var right) => new Value.Bool(
                !Evaluate(left, parameters).Equals(Evaluate(right, parameters))),
            Operator.GreaterThan(var left, var right) => new Value.Bool(
                Compare(Evaluate(left, parameters), Evaluate(right, parameters)) > 0),
            Operator.LessThan(var left, var right) => new Value.Bool(
                Compare(Evaluate(left, parameters), Evaluate(right, parameters)) < 0),
            Operator.GreaterThanOrEqual(var left, var right) => new Value.Bool(
                Compare(Evaluate(left, parameters), Evaluate(right, parameters)) >= 0),
            Operator.LessThanOrEqual(var left, var right) => new Value.Bool(
                Compare(Evaluate(left, parameters), Evaluate(right, parameters)) <= 0),
            ArrayExpression => throw new InvalidOperationException("array is not a scalar"),
            _ => throw new InvalidOperationException("empty expression")
        };

    private bool Contains(Value left, IExpression right, IReadOnlyDictionary<string, Value>? parameters) =>
        right is ArrayExpression arr
            ? arr.Elements.Any(element => left.Equals(Evaluate(element, parameters)))
            : throw new InvalidOperationException("empty in");

    private static bool IsTrue(Value value) => value is Value.Bool(true);

    static bool Like(Value text, Value pattern) => (text, pattern) switch
    {
        (Value.Null, _) or (_, Value.Null) => false,
        (Value.Text(var value), Value.Text(var like)) => Wild(value, like),
        _ => throw new InvalidOperationException("empty like")
    };

    static bool Wild(string text, string pattern)
    {
        var t = 0;
        var p = 0;
        var starT = -1;
        var starP = -1;
        while (t < text.Length)
        {
            if (p < pattern.Length && (pattern[p] == '_' || pattern[p] == text[t]))
            {
                t++;
                p++;
            }
            else if (p < pattern.Length && pattern[p] == '%')
            {
                starP = p++;
                starT = t;
            }
            else if (starP >= 0)
            {
                p = starP + 1;
                t = ++starT;
            }
            else return false;
        }
        while (p < pattern.Length && pattern[p] == '%') p++;
        return p == pattern.Length;
    }

    static bool Between(Value value, Value low, Value high) =>
        value is Value.Null || low is Value.Null || high is Value.Null
            ? false
            : Compare(value, low) >= 0 && Compare(value, high) <= 0;

    private static int Compare(Value left, Value right) => (left, right) switch
    {
        (Value.Int(var a), Value.Int(var b)) => a.CompareTo(b),
        (Value.Float(var a), Value.Float(var b)) => a.CompareTo(b),
        (Value.Decimal(var a), Value.Decimal(var b)) => a.CompareTo(b),
        (Value.Int(var a), Value.Float(var b)) => ((double)a).CompareTo(b),
        (Value.Float(var a), Value.Int(var b)) => a.CompareTo(b),
        (Value.Int(var a), Value.Decimal(var b)) => ((decimal)a).CompareTo(b),
        (Value.Decimal(var a), Value.Int(var b)) => a.CompareTo(b),
        (Value.Text(var a), Value.Text(var b)) => string.CompareOrdinal(a, b),
        (Value.DateTime(var a), Value.DateTime(var b)) => a.CompareTo(b),
        _ => throw new InvalidOperationException("empty compare")
    };
}
