namespace Datasources.CodeGenerators;

public sealed class DatasourceFieldOverlay
{
    public required string Name { get; init; }
    public bool? IsReadonly { get; init; }
    public bool? IsUnique { get; init; }
    public string? Mapping { get; init; }
    public bool? IsFixedId { get; init; }
}
