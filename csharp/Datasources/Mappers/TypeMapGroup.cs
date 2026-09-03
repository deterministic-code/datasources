namespace Datasources;

public sealed class TypeMapGroup
{
    public required string Key { get; init; }
    public required IReadOnlyDictionary<string, TypeMapField> Fields { get; init; }
}
