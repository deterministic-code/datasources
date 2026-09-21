namespace Datasources;

public sealed record UniqueConstraint(string Name, IReadOnlyList<string> Columns) : ITableConstraint;
