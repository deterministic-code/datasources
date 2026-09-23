namespace Datasources;

public delegate IExpression Comparison(string column, Value? value);

public static class Cmp
{
    public static IExpression eq(string column, Value? value) => new Operator.Equals(column, value);
    public static IExpression ne(string column, Value? value) => new Operator.NotEquals(column, value);
    public static IExpression gt(string column, Value? value) => new Operator.GreaterThan(column, value);
    public static IExpression lt(string column, Value? value) => new Operator.LessThan(column, value);
    public static IExpression ge(string column, Value? value) => new Operator.GreaterThanOrEqual(column, value);
    public static IExpression le(string column, Value? value) => new Operator.LessThanOrEqual(column, value);
}
