namespace Datasources.Operator;

using Datasources;

public sealed class Equals : IExpression
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public Equals(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public Equals(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed record NotEquals(IExpression Left, IExpression Right) : IExpression
{
    public NotEquals(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record GreaterThan(IExpression Left, IExpression Right) : IExpression
{
    public GreaterThan(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record LessThan(IExpression Left, IExpression Right) : IExpression
{
    public LessThan(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record GreaterThanOrEqual(IExpression Left, IExpression Right) : IExpression
{
    public GreaterThanOrEqual(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record LessThanOrEqual(IExpression Left, IExpression Right) : IExpression
{
    public LessThanOrEqual(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record And(IExpression Left, IExpression Right) : IExpression;

public sealed record Or(IExpression Left, IExpression Right) : IExpression;

public sealed record In(IExpression Left, IExpression Right) : IExpression
{
    public In(string field, IReadOnlyList<Value> values)
        : this(
            new Identifier(field),
            new ArrayExpression(values.Select(value => (IExpression)new Literal(value)).ToArray())) { }
}

public sealed record Not(IExpression Argument) : IExpression;

public sealed record IsNull(IExpression Argument) : IExpression;

public sealed record IsNotNull(IExpression Argument) : IExpression;

public sealed record Like(IExpression Left, IExpression Right) : IExpression
{
    public Like(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record NotLike(IExpression Left, IExpression Right) : IExpression
{
    public NotLike(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
}

public sealed record Between(IExpression Value, IExpression Low, IExpression High) : IExpression
{
    public Between(string field, Datasources.Value? low, Datasources.Value? high)
        : this(
            new Identifier(field),
            new Literal(low ?? Datasources.Value.Null.Instance),
            new Literal(high ?? Datasources.Value.Null.Instance)) { }
}

public sealed record NotBetween(IExpression Value, IExpression Low, IExpression High) : IExpression
{
    public NotBetween(string field, Datasources.Value? low, Datasources.Value? high)
        : this(
            new Identifier(field),
            new Literal(low ?? Datasources.Value.Null.Instance),
            new Literal(high ?? Datasources.Value.Null.Instance)) { }
}

public sealed record NotIn(IExpression Left, IExpression Right) : IExpression
{
    public NotIn(string field, IReadOnlyList<Value> values)
        : this(
            new Identifier(field),
            new ArrayExpression(values.Select(value => (IExpression)new Literal(value)).ToArray())) { }
}

public sealed record Exists(IExpression Argument) : IExpression;
