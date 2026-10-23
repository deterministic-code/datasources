using Deterministic.Generators.Common;

namespace Datasources.CodeGenerators;

public static class SpecParser
{
    public const string TypesYaml = "types.yaml";
    public const string DatasourceYaml = "datasource.yaml";
    public const string SeedsYaml = "datasource_seeds.yaml";

    public static ParsedSpec Parse(IReadOnlyDictionary<string, string> files)
    {
        var spec = new DeterministicParser(new MemoryReader(files))
            .Parse(new Dictionary<string, string>())
            .GetAwaiter()
            .GetResult();
        var parsed = new ParsedSpec
        {
            Types = spec.Types.Select(MapType).ToList(),
            ExpandedTypes = spec.ExpandedTypes.Select(MapType).ToList(),
            Datasource = spec.Datasource.Select(MapTable).ToList(),
            DatasourceSeeds = spec.DatasourceSeeds.ToDictionary(
                pair => pair.Key,
                pair => (IReadOnlyList<SeedRow>)pair.Value.Select(MapSeed).ToList(),
                StringComparer.Ordinal),
        };
        return parsed;
    }

    static SpecType MapType(TypeModel type) => new()
    {
        Name = type.Name,
        Tags = type.Tags,
        Kind = type.Kind,
        Inherits = type.Inherits,
        Union = type.Union,
        Mapping = type.Mapping,
        Extract = type.Extract,
        RemoveFields = type.RemoveFields,
        Ids = type.Ids,
        Fields = type.Fields.Select(MapField).ToList(),
    };

    static SpecTypeField MapField(TypeField field)
    {
        var (text, pair) = Refs(field.References);
        return new()
        {
            Name = field.Name,
            Type = field.Type,
            Kind = field.Kind,
            Base = field.Base,
            IsArray = field.IsArray,
            IsMap = field.IsMap,
            IsNullable = field.IsNullable,
            IsId = field.IsId == true,
            Size = field.Size,
            References = text,
            ReferencePair = pair,
            HasDefault = field.HasDefault,
            DefaultValue = field.DefaultValue,
        };
    }

    static DatasourceTable MapTable(Deterministic.Generators.Common.DatasourceTable table) => new()
    {
        Name = table.Name,
        Mapping = table.Mapping,
        Fields = table.Fields.Select(MapOverlay).ToList(),
        Indexes = table.Indexes.Select(index => new DatasourceIndex
        {
            Name = index.Name,
            Fields = index.Fields,
            IsUnique = index.IsUnique,
        }).ToList(),
        UniqueIndexFields = table.UniqueIndexFields,
        UseOptimisticConcurrency = table.UseOptimisticConcurrency,
    };

    static DatasourceFieldOverlay MapOverlay(Deterministic.Generators.Common.DatasourceFieldOverlay field) => new()
    {
        Name = field.Name,
        IsReadonly = field.IsReadonly,
        IsUnique = field.IsUnique,
        Mapping = field.Mapping,
        IsFixedId = field.IsFixedId,
        IncludeInCreate = field.IncludeInCreate,
        IncludeInUpdate = field.IncludeInUpdate,
        IsOptimisticConcurrency = field.IsOptimisticConcurrency,
        UseNativeRowVersion = field.UseNativeRowVersion,
    };

    static SeedRow MapSeed(Deterministic.Generators.Common.SeedRow row) => new()
    {
        Id = row.Id,
        Row = row.Row,
    };

    static (string? Text, IReadOnlyList<string>? Pair) Refs(object? raw) => raw switch
    {
        string text => (text, null),
        string[] { Length: 2 } pair => (null, pair),
        IReadOnlyList<string> { Count: 2 } list => (null, list.ToList()),
        _ => (null, null)
    };
}
