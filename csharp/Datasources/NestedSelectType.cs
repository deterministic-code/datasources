namespace Datasources;

public sealed class NestedSelectType
{
    public required string Name { get; init; }
    public required string Key { get; init; }
    public IReadOnlyList<string> Properties { get; init; } = [];
    public IReadOnlyList<NestedSelectRelation> Relations { get; init; } = [];
}
