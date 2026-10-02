namespace Datasources.Operator;

using Datasources;

public sealed class Equals : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public Equals(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public Equals(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class NotEquals : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public NotEquals(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public NotEquals(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class GreaterThan : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public GreaterThan(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public GreaterThan(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class LessThan : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public LessThan(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public LessThan(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class GreaterThanOrEqual : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public GreaterThanOrEqual(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public GreaterThanOrEqual(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class LessThanOrEqual : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public LessThanOrEqual(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public LessThanOrEqual(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class And : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public And(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class Or : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public Or(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class In : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public In(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public In(string field, IReadOnlyList<Value> values)
        : this(
            new Identifier(field),
            new ArrayExpression(values.Select(value => (IExpression)new Literal(value)).ToArray())) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class Not : Condition
{
    public IExpression Argument { get; }
    public Not(IExpression argument) => Argument = argument;
    public void Deconstruct(out IExpression argument) => argument = Argument;
}

public sealed class IsNull : Condition
{
    public IExpression Argument { get; }
    public IsNull(IExpression argument) => Argument = argument;
    public void Deconstruct(out IExpression argument) => argument = Argument;
}

public sealed class IsNotNull : Condition
{
    public IExpression Argument { get; }
    public IsNotNull(IExpression argument) => Argument = argument;
    public void Deconstruct(out IExpression argument) => argument = Argument;
}

public sealed class Like : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public Like(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public Like(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class NotLike : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public NotLike(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public NotLike(string field, Value? value)
        : this(new Identifier(field), new Literal(value ?? Value.Null.Instance)) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class Between : Condition
{
    public IExpression Value { get; }
    public IExpression Low { get; }
    public IExpression High { get; }
    public Between(IExpression value, IExpression low, IExpression high) =>
        (Value, Low, High) = (value, low, high);
    public Between(string field, Datasources.Value? low, Datasources.Value? high)
        : this(
            new Identifier(field),
            new Literal(low ?? Datasources.Value.Null.Instance),
            new Literal(high ?? Datasources.Value.Null.Instance)) { }
    public void Deconstruct(out IExpression value, out IExpression low, out IExpression high) =>
        (value, low, high) = (Value, Low, High);
}

public sealed class NotBetween : Condition
{
    public IExpression Value { get; }
    public IExpression Low { get; }
    public IExpression High { get; }
    public NotBetween(IExpression value, IExpression low, IExpression high) =>
        (Value, Low, High) = (value, low, high);
    public NotBetween(string field, Datasources.Value? low, Datasources.Value? high)
        : this(
            new Identifier(field),
            new Literal(low ?? Datasources.Value.Null.Instance),
            new Literal(high ?? Datasources.Value.Null.Instance)) { }
    public void Deconstruct(out IExpression value, out IExpression low, out IExpression high) =>
        (value, low, high) = (Value, Low, High);
}

public sealed class NotIn : Condition
{
    public IExpression Left { get; }
    public IExpression Right { get; }
    public NotIn(IExpression left, IExpression right) => (Left, Right) = (left, right);
    public NotIn(string field, IReadOnlyList<Value> values)
        : this(
            new Identifier(field),
            new ArrayExpression(values.Select(value => (IExpression)new Literal(value)).ToArray())) { }
    public void Deconstruct(out IExpression left, out IExpression right) =>
        (left, right) = (Left, Right);
}

public sealed class Exists : Condition
{
    public IExpression Argument { get; }
    public Exists(IExpression argument) => Argument = argument;
    public void Deconstruct(out IExpression argument) => argument = Argument;
}
