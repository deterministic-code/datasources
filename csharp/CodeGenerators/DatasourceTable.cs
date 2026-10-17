namespace Datasources.CodeGenerators;

public sealed class DatasourceTable
{
    public required string Name { get; init; }
    public string? Mapping { get; init; }
    public IReadOnlyList<DatasourceFieldOverlay> Fields { get; init; } = [];
    public IReadOnlyList<DatasourceIndex> Indexes { get; init; } = [];
    public IReadOnlyList<string> UniqueIndexFields { get; init; } = [];
    public bool? UseOptimisticConcurrency { get; init; }
}
