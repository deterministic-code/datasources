namespace Datasources.CodeGenerators;

public sealed class SeedRow
{
    public required int Id { get; init; }
    public required IReadOnlyDictionary<string, object?> Row { get; init; }
}
