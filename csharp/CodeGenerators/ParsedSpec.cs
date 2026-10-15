namespace Datasources.CodeGenerators;

public sealed class ParsedSpec
{
    public IReadOnlyList<SpecType> Types { get; init; } = [];
    public IReadOnlyList<SpecType> ExpandedTypes { get; init; } = [];
    public IReadOnlyList<DatasourceTable> Datasource { get; init; } = [];
    public IReadOnlyDictionary<string, IReadOnlyList<SeedRow>> DatasourceSeeds { get; init; } =
        new Dictionary<string, IReadOnlyList<SeedRow>>();
}
