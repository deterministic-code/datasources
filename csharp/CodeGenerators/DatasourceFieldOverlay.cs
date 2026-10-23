namespace Datasources.CodeGenerators;

public sealed class DatasourceFieldOverlay
{
    public required string Name { get; init; }
    public bool? IsReadonly { get; init; }
    public bool? IsUnique { get; init; }
    public string? Mapping { get; init; }
    public bool? IsFixedId { get; init; }
    public bool? IncludeInCreate { get; init; }
    public bool? IncludeInUpdate { get; init; }
    public bool? IsOptimisticConcurrency { get; init; }
    public bool? UseNativeRowVersion { get; init; }
}
