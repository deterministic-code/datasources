namespace Datasources.CodeGenerators;

public sealed class DatasourceIndex
{
    public required string Name { get; init; }
    public required IReadOnlyList<string> Fields { get; init; }
    public required bool IsUnique { get; init; }
}
