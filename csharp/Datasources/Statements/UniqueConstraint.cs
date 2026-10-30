namespace Datasources;

public sealed record UniqueConstraint(string Name, IReadOnlyList<string> Columns) : ITableConstraint
{
    public static string NameOf(string entity, StatementNaming naming, params string[] parts) =>
        ConstraintName.Of(entity, naming, parts);
}
