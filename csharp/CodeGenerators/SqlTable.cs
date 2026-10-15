namespace Datasources.CodeGenerators;

public sealed record SqlTable
{
    public required string Name { get; init; }
    public IReadOnlyList<string> Tags { get; init; } = [];
    public string Kind { get; init; } = "inherit";
    public string? Inherits { get; init; }
    public IReadOnlyList<string>? Ids { get; init; }
    public IReadOnlyList<SpecTypeField> Fields { get; init; } = [];
    public IReadOnlyList<DatasourceIndex> Indexes { get; init; } = [];
    public IReadOnlyList<string> UniqueIndexFields { get; init; } = [];
    public string? Mapping { get; init; }
    public IReadOnlyList<DatasourceFieldOverlay> Overlays { get; init; } = [];
    public string TableName { get; init; } = "";

    public SpecType AsType() => new()
    {
        Name = Name,
        Tags = Tags,
        Kind = Kind,
        Inherits = Inherits,
        Ids = Ids,
        Fields = Fields,
    };
}
