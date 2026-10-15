namespace Datasources.CodeGenerators;

public static class SpecParser
{
    public const string TypesYaml = "types.yaml";
    public const string DatasourceYaml = "datasource.yaml";
    public const string SeedsYaml = "datasource_seeds.yaml";

    public static ParsedSpec Parse(IReadOnlyDictionary<string, string> files)
    {
        var types = files.TryGetValue(TypesYaml, out var typesYaml) ? ParseTypes(typesYaml) : [];
        var expanded = ExpandTypes.Of(types);
        var datasource = files.TryGetValue(DatasourceYaml, out var dsYaml) ? ParseDatasource(dsYaml, expanded) : [];
        var seeds = files.TryGetValue(SeedsYaml, out var seedsYaml)
            ? ParseSeeds(seedsYaml)
            : new Dictionary<string, IReadOnlyList<SeedRow>>();
        return new ParsedSpec
        {
            Types = types,
            ExpandedTypes = expanded,
            Datasource = datasource,
            DatasourceSeeds = seeds,
        };
    }

    static IReadOnlyList<SpecType> ParseTypes(string yaml) =>
        YamlCursor.FromYaml(yaml).NamedList("types").Select(entry =>
        {
            var node = entry.Node;
            var mapping = MappingOf(node);
            var extract = node.Strings("extract");
            var remove = node.Strings("remove_fields");
            var ids = node.Strings("ids");
            return new SpecType
            {
                Name = entry.Name,
                Tags = node.Strings("tags"),
                Kind = node.Str("inherits") is not null ? "inherit" : node.Child("union").Value is List<object> ? "union" : "shaped",
                Inherits = node.Str("inherits"),
                Union = node.Child("union").Value is List<object> ? node.Strings("union") : null,
                Mapping = mapping,
                Extract = extract.Count > 0 ? extract : null,
                RemoveFields = remove.Count > 0 ? remove : null,
                Ids = ids.Count > 0 ? ids : null,
                Fields = node.NamedList("fields").Select(field => ReadField(field.Name, field.Node)).ToList(),
            };
        }).ToList();

    static SpecTypeField ReadField(string name, YamlCursor node)
    {
        var raw = node.Str("type") ?? "string";
        var parsed = FieldType.Parse(raw);
        var (references, pair) = ReferencesOf(node);
        return new SpecTypeField
        {
            Name = name,
            Type = raw,
            Kind = parsed.Kind,
            Base = parsed.Base,
            IsArray = parsed.IsArray,
            IsMap = parsed.IsMap,
            IsNullable = node.Bool("is_nullable"),
            IsId = node.Bool("is_id"),
            Size = SizeOf(node),
            References = references,
            ReferencePair = pair,
            HasDefault = node.Has("default_value"),
            DefaultValue = node.Has("default_value") ? node.Literal("default_value") : null,
        };
    }

    static IReadOnlyList<DatasourceTable> ParseDatasource(string yaml, IReadOnlyList<SpecType> types)
    {
        var root = YamlCursor.FromYaml(yaml);
        var tables = root.NamedList("types").Select(entry =>
        {
            var unique = new List<string>();
            var indexes = new List<DatasourceIndex>();
            foreach (var index in entry.Node.NamedList("indexes"))
            {
                var fields = index.Node.Child("fields").Value is List<object> list
                    ? list.OfType<string>().ToList()
                    : new List<string>();
                var isUnique = index.Node.Bool("is_unique");
                indexes.Add(new DatasourceIndex { Name = index.Name, Fields = fields, IsUnique = isUnique });
                if (isUnique && fields.Count == 1) unique.Add(fields[0]);
            }
            var overlays = entry.Node.NamedList("fields").Select(field => new DatasourceFieldOverlay
            {
                Name = field.Name,
                IsReadonly = field.Node.Has("is_readonly") ? field.Node.Bool("is_readonly") : null,
                IsUnique = field.Node.Has("is_unique") ? field.Node.Bool("is_unique") : null,
                Mapping = field.Node.Str("mapping"),
                IsFixedId = field.Node.Has("is_fixed_id") ? field.Node.Bool("is_fixed_id") : null,
            }).ToList();
            return new DatasourceTable
            {
                Name = entry.Name,
                Mapping = entry.Node.Str("mapping"),
                Fields = overlays,
                Indexes = indexes,
                UniqueIndexFields = unique,
            };
        }).ToList();
        var filterExpr = TypesFilter(root);
        if (filterExpr is null) return tables;
        var selected = types.Where(filterExpr).Select(t => t.Name).ToHashSet(StringComparer.Ordinal);
        var overlayNames = tables.Select(t => t.Name).ToHashSet(StringComparer.Ordinal);
        var extras = types
            .Where(t => selected.Contains(t.Name) && !overlayNames.Contains(t.Name))
            .Select(t => new DatasourceTable { Name = t.Name });
        return [.. tables.Where(t => selected.Contains(t.Name)), .. extras];
    }

    static Dictionary<string, IReadOnlyList<SeedRow>> ParseSeeds(string yaml)
    {
        var byTable = new Dictionary<string, IReadOnlyList<SeedRow>>(StringComparer.Ordinal);
        foreach (var entry in YamlCursor.FromYaml(yaml).NamedList("seeds"))
        {
            byTable[entry.Name] = entry.Node.NamedItems().Select(row => new SeedRow
            {
                Id = ParseSeedKey(row.Name),
                Row = SeedCells(row.Node),
            }).ToList();
        }
        return byTable;
    }

    static Func<SpecType, bool>? TypesFilter(YamlCursor root)
    {
        foreach (var entry in root.Child("includes").Items())
        {
            if (entry.Child("types").MapRecord())
                return CompileFilter.Types(entry.Child("types").Str("filter"));
        }
        return null;
    }

    static int ParseSeedKey(string rowKey)
    {
        if (rowKey.Length < 3 || !rowKey.StartsWith("id", StringComparison.Ordinal) || !int.TryParse(rowKey[2..], out var id))
            throw new InvalidOperationException($"Invalid seed row key \"{rowKey}\": expected pattern /^id\\d+$/");
        return id;
    }

    static Dictionary<string, object?> SeedCells(YamlCursor node)
    {
        var outCells = new Dictionary<string, object?>(StringComparer.Ordinal);
        if (node.Value is not Dictionary<object, object> rec) return outCells;
        foreach (var key in rec.Keys.OfType<string>())
        {
            var value = node.Literal(key);
            if (value is not null || node.Has(key)) outCells[key] = value;
        }
        return outCells;
    }

    static IReadOnlyDictionary<string, string>? MappingOf(YamlCursor node)
    {
        if (node.Child("mapping").Value is not Dictionary<object, object> rec) return null;
        var map = rec.Where(p => p.Key is string && p.Value is string)
            .ToDictionary(p => (string)p.Key, p => (string)p.Value, StringComparer.Ordinal);
        return map.Count > 0 ? map : null;
    }

    static (string? Text, IReadOnlyList<string>? Pair) ReferencesOf(YamlCursor node)
    {
        var raw = node.Child("references").Value;
        if (raw is string text) return (text, null);
        if (raw is List<object> { Count: 2 } list && list[0] is string a && list[1] is string b)
            return (null, [a, b]);
        return (null, null);
    }

    static object? SizeOf(YamlCursor node)
    {
        var raw = node.Child("size").Value;
        if (raw is string { } text && text == "unlimited") return "unlimited";
        if (raw is int or long or short or byte) return Convert.ToInt32(raw);
        if (raw is List<object> { Count: 2 } list)
            return new[] { Convert.ToInt32(list[0]), Convert.ToInt32(list[1]) };
        if (raw is double d && d == Math.Floor(d)) return Convert.ToInt32(d);
        return null;
    }
}

file static class YamlMap
{
    public static bool MapRecord(this YamlCursor node) => node.Value is Dictionary<object, object>;
}
