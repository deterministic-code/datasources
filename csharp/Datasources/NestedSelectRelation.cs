namespace Datasources;

public sealed class NestedSelectRelation
{
    public required string Name { get; init; }
    public required NestedSelectRelationKind Kind { get; init; }
    public required string From { get; init; }
    public required string To { get; init; }
    public required NestedSelectType Target { get; init; }
}
