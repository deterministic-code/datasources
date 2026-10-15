namespace Datasources.CodeGenerators;

public sealed class SpecType
{
    public required string Name { get; init; }
    public IReadOnlyList<string> Tags { get; init; } = [];
    public string Kind { get; init; } = "shaped";
    public string? Inherits { get; init; }
    public IReadOnlyList<string>? Union { get; init; }
    public IReadOnlyDictionary<string, string>? Mapping { get; init; }
    public IReadOnlyList<string>? Extract { get; init; }
    public IReadOnlyList<string>? RemoveFields { get; init; }
    public IReadOnlyList<string>? Ids { get; init; }
    public IReadOnlyList<SpecTypeField> Fields { get; init; } = [];
}
