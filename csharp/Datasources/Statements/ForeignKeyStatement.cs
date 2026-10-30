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

    public static string NameOf(string entity, StatementNaming naming, params string[] parts) =>
        ConstraintName.Of(entity, naming, parts);
}
