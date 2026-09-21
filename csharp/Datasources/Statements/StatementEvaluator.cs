namespace Datasources;

public sealed class StatementEvaluator
{
    public Value Evaluate(IExpression expression, IReadOnlyDictionary<string, Value>? parameters) =>
        expression switch
        {
            WhereClause where => Evaluate(where.Condition, parameters),
            Where => throw new InvalidOperationException("text where is not evaluable"),
            Identifier id => parameters is not null && parameters.TryGetValue(id.Name, out var value)
                ? value : Value.Null.Instance,
            Literal lit => lit.Value,
            Operator.Not not => new Value.Bool(!IsTrue(Evaluate(not.Argument, parameters))),
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
