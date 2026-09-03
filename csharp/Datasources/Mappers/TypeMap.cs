namespace Datasources;

public sealed class TypeMap
{
    public required string Key { get; init; }
    public required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
}
