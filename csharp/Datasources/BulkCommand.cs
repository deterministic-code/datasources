namespace Datasources;

public sealed record BulkCommand(string Text, IReadOnlyList<Parameter> Parameters);
