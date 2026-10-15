namespace Datasources.CodeGenerators;

public static class ExpandTypes
{
    static readonly SpecTypeField[] SetFields =
    [
        new()
        {
            Name = "id",
            Type = "integer",
            Kind = "primitive",
            Base = "integer",
            IsArray = false,
            IsMap = false,
            IsNullable = false,
        },
    ];

    public static IReadOnlyList<SpecType> Of(IReadOnlyList<SpecType> types)
    {
        var byName = types.ToDictionary(t => t.Name, StringComparer.Ordinal);
        var cache = new Dictionary<string, IReadOnlyList<SpecTypeField>>(StringComparer.Ordinal);

        IReadOnlyList<SpecTypeField> Resolve(string name, HashSet<string> stack)
        {
            if (cache.TryGetValue(name, out var hit)) return hit;
            if (name == "set")
            {
                cache[name] = SetFields;
                return SetFields;
            }
            if (!stack.Add(name))
                throw new InvalidOperationException($"circular inherit/union involving \"{name}\"");
            if (!byName.TryGetValue(name, out var type))
            {
                stack.Remove(name);
                return [];
            }
            List<SpecTypeField> Compose(string source)
            {
                var composed = Resolve(source, stack).Select(f => Clone(f, source, f.Name)).ToList();
                return byName.GetValueOrDefault(source) is { Inherits: "dictionary" } srcType
                    ? composed.Where(f => !SpecTables.IsOwnerIdentityRef(f, srcType.Name, byName)).ToList()
                    : composed;
            }

            var skipSetId = type.Inherits == "set" && SpecTables.HasAuthoredIdentity(type);
            var inherited = new List<SpecTypeField>();
            if (type.Inherits is { } parent && !skipSetId) inherited.AddRange(Compose(parent));
            foreach (var member in type.Union ?? []) inherited.AddRange(Compose(member));
            stack.Remove(name);
            var merged = Keep(Rename(Extract(inherited, type.Extract), type.Mapping), type.RemoveFields)
                .Concat(type.Fields.Select(f => Clone(f, name, f.Name)))
                .ToList();
            AssertUnique(merged, name);
            var result = merged.Select(f => Clone(f, null, null)).ToList();
            cache[name] = result;
            return result;
        }

        IReadOnlyList<string>? ResolveIds(string name, HashSet<string> stack)
        {
            if (!stack.Add(name) || !byName.TryGetValue(name, out var type)) return null;
            if (type.Ids is { Count: > 0 } ids) return ids;
            if (type.Fields.Any(f => f.IsId)) return null;
            if (type.Inherits is { } parent && parent is not "set" and not "dictionary")
                return ResolveIds(parent, stack);
            return null;
        }

        return types.Select(type =>
        {
            var ids = type.Ids ?? ResolveIds(type.Name, []);
            return new SpecType
            {
                Name = type.Name,
                Tags = type.Tags,
                Kind = type.Kind,
                Inherits = type.Inherits,
                Union = type.Union,
                Mapping = type.Mapping,
                Extract = type.Extract,
                RemoveFields = type.RemoveFields,
                Ids = ids is { Count: > 0 } ? ids : type.Ids,
                Fields = Resolve(type.Name, []),
            };
        }).ToList();
    }

    static SpecTypeField Clone(SpecTypeField field, string? source, string? original) => new()
    {
        Name = field.Name,
        Type = field.Type,
        Kind = field.Kind,
        Base = field.Base,
        IsArray = field.IsArray,
        IsMap = field.IsMap,
        IsNullable = field.IsNullable,
        IsId = field.IsId,
        Size = field.Size,
        References = field.References,
        ReferencePair = field.ReferencePair,
        DefaultValue = field.DefaultValue,
        HasDefault = field.HasDefault,
        Source = source,
        Original = original,
    };

    static List<SpecTypeField> Extract(List<SpecTypeField> fields, IReadOnlyList<string>? extract)
    {
        if (extract is not { Count: > 0 }) return fields;
        var allow = new Dictionary<string, HashSet<string>>(StringComparer.Ordinal);
        foreach (var item in extract)
        {
            var split = SpecTables.SplitDot(item);
            if (split is null) continue;
            if (!allow.TryGetValue(split.Value.Left, out var set))
                allow[split.Value.Left] = set = [];
            set.Add(split.Value.Right);
        }
        return fields.Where(f => !allow.TryGetValue(f.Source ?? "", out var keep) || keep.Contains(f.Name)).ToList();
    }

    static List<SpecTypeField> Rename(List<SpecTypeField> fields, IReadOnlyDictionary<string, string>? mapping) =>
        mapping is null
            ? fields
            : fields.Select(f =>
            {
                var name = mapping.GetValueOrDefault($"{f.Source}.{f.Name}") ?? mapping.GetValueOrDefault(f.Name);
                return name is null ? f : Clone(f with { Name = name }, f.Source, f.Original);
            }).ToList();

    static List<SpecTypeField> Keep(List<SpecTypeField> fields, IReadOnlyList<string>? remove) =>
        remove is not { Count: > 0 }
            ? fields
            : fields.Where(f => !remove.Any(item =>
            {
                var split = SpecTables.SplitDot(item);
                return split is { } pair ? f.Name == pair.Right && f.Source == pair.Left : f.Name == item;
            })).ToList();

    static void AssertUnique(List<SpecTypeField> fields, string typeName)
    {
        var seen = new Dictionary<string, SpecTypeField>(StringComparer.Ordinal);
        foreach (var field in fields)
        {
            if (seen.TryGetValue(field.Name, out var prev))
                throw new InvalidOperationException(
                    $"{prev.Source}.{prev.Original} and {field.Source}.{field.Original} collide on the composed shape of {typeName}; use mapping or remove_fields and give new names");
            seen[field.Name] = field;
        }
    }
}
