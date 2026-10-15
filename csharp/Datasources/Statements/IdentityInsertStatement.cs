namespace Datasources;

public sealed record IdentityInsertStatement(
    string Table,
    string IdColumn,
    IReadOnlyList<InsertStatement> Inserts,
    bool Identity = false) : IIdentityInsertStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
