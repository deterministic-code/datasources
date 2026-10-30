namespace Datasources;

public sealed record SqlSeedStatement(
    string Table,
    IReadOnlyList<SqlSeedColumn> Columns,
    string Kind = "insert",
    bool Identity = false,
    string? IdColumn = null) : ISqlSeedStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
