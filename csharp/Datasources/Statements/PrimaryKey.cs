namespace Datasources;

public sealed record PrimaryKey(string Name, IReadOnlyList<string> Columns) : ITableConstraint;
