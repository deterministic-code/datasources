namespace Datasources;

public sealed record ForeignKeyStatement(
    string Table,
    string Name,
    IReadOnlyList<string> Columns,
    string ReferencesTable,
    IReadOnlyList<string> ReferencesColumns,
    string Kind = "add",
    ReferentialAction? OnDelete = null,
    ReferentialAction? OnUpdate = null) : IForeignKeyStatement
{
    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);
}
