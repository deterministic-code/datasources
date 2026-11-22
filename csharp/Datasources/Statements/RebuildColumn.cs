namespace Datasources;

public sealed record RebuildColumn(string Name, string? Source = null, string? Cast = null, string? Literal = null);
