namespace Datasources;

public delegate Condition Comparison(string column, Value? value);

public static class Cmp
{
    public static Condition eq(string column, Value? value) => new Operator.Equals(column, value);
    public static Condition ne(string column, Value? value) => new Operator.NotEquals(column, value);
    public static Condition gt(string column, Value? value) => new Operator.GreaterThan(column, value);
    public static Condition lt(string column, Value? value) => new Operator.LessThan(column, value);
    public static Condition ge(string column, Value? value) => new Operator.GreaterThanOrEqual(column, value);
    public static Condition le(string column, Value? value) => new Operator.LessThanOrEqual(column, value);
}
