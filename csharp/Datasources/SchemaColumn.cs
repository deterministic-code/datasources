namespace Datasources;

public sealed record SchemaColumn(string Type, bool Nullable = false, bool HasDefault = false);
