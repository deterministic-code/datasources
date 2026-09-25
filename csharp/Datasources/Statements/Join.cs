namespace Datasources;

public enum JoinKind
{
    Inner,
    Left,
    Right,
    Full,
    Cross,
}

public sealed record Join(JoinKind Kind, string Table, IExpression? On)
{
    public Join(JoinKind kind, string table, string left, string right)
        : this(kind, table, new Operator.Equals(new Identifier(left), new Identifier(right))) { }

    public static Join Inner(string table, string left, string right) => new(JoinKind.Inner, table, left, right);
    public static Join Left(string table, string left, string right) => new(JoinKind.Left, table, left, right);
    public static Join Right(string table, string left, string right) => new(JoinKind.Right, table, left, right);
    public static Join Full(string table, string left, string right) => new(JoinKind.Full, table, left, right);
    public static Join Cross(string table) => new(JoinKind.Cross, table, (IExpression?)null);
}
