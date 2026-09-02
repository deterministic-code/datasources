namespace Datasources.Builders;

public enum JoinKind
{
    Inner,
    Left,
    Right,
}

public sealed record Join(JoinKind Kind, string Table, string LeftColumn, string RightColumn)
{
    public static Join Inner(string table, string left, string right) => new(JoinKind.Inner, table, left, right);
    public static Join Left(string table, string left, string right) => new(JoinKind.Left, table, left, right);
    public static Join Right(string table, string left, string right) => new(JoinKind.Right, table, left, right);
}
