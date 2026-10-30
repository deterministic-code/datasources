namespace Datasources;

public enum ReferentialAction
{
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}

public sealed record ForeignKey(
    string Name,
    IReadOnlyList<string> Columns,
    string ReferencesTable,
    IReadOnlyList<string> ReferencesColumns,
    ReferentialAction? OnDelete = null,
    ReferentialAction? OnUpdate = null) : ITableConstraint
{
    public static string NameOf(string entity, StatementNaming naming, params string[] parts) =>
        ConstraintName.Of(entity, naming, parts);
}
