namespace Datasources.CodeGenerators;

public sealed record SpecTypeField
{
    public required string Name { get; init; }
    public required string Type { get; init; }
    public required string Kind { get; init; }
    public required string Base { get; init; }
    public required bool IsArray { get; init; }
    public bool IsMap { get; init; }
    public required bool IsNullable { get; init; }
    public bool IsId { get; init; }
    public object? Size { get; init; }
    public string? References { get; init; }
    public IReadOnlyList<string>? ReferencePair { get; init; }
    public object? DefaultValue { get; init; }
    public bool HasDefault { get; init; }
    public string? Source { get; init; }
    public string? Original { get; init; }

    public SpecField ConverterField() => new()
    {
        Type = Type,
        Name = Name,
        Size = Size,
        DefaultValue = DefaultValue,
    };

    public object ReferencesValue() =>
        ReferencePair is { Count: 2 } pair ? pair : References ?? "";
}
