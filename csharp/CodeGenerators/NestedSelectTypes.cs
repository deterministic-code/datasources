namespace Datasources.CodeGenerators;

public static class NestedSelectTypes
{
    static readonly ChangeCasingMapper Pascal = new(Casing.Snake, Casing.Pascal);
    static readonly PluralizerMapper Plural = new();

    public static NestedSelectType FromSpec(IReadOnlyList<SpecType> types, string root)
    {
        var catalog = types
            .Where(type => type.Tags.Contains("datasource_type") || types.All(item => item.Tags.Count == 0))
            .ToDictionary(type => type.Name, StringComparer.Ordinal);
        var name = catalog.ContainsKey(root)
            ? root
            : catalog.Keys.FirstOrDefault(item => Entity(item) == root)
                ?? throw new InvalidOperationException($"Unknown nested select type \"{root}\"");
        return Build(catalog[name], catalog, []);
    }

    static NestedSelectType Build(
        SpecType type,
        IReadOnlyDictionary<string, SpecType> catalog,
        HashSet<string> ancestors)
    {
        var next = new HashSet<string>(ancestors, StringComparer.Ordinal) { type.Name };
        var key = Entity(type.Fields.FirstOrDefault(field => field.IsId)?.Name ?? type.Ids?.FirstOrDefault() ?? "id");
        var properties = type.Fields
            .Where(field => field.References is null && field.ReferencePair is null)
            .Select(field => Entity(field.Name))
            .ToList();
        var objects = type.Fields
            .Select(field => (field, target: Target(field)))
            .Where(pair => pair.target.Table.Length > 0 && catalog.ContainsKey(pair.target.Table) && !next.Contains(pair.target.Table))
            .Select(pair => new NestedSelectRelation
            {
                Name = RelationName(pair.field.Name, pair.target.Table),
                Kind = NestedSelectRelationKind.Object,
                From = Entity(pair.field.Name),
                To = Entity(pair.target.Column),
                Target = Build(catalog[pair.target.Table], catalog, next),
            });
        var lists = catalog.Values
            .Where(child => !next.Contains(child.Name))
            .SelectMany(child => child.Fields
                .Select(field => (child, field, target: Target(field)))
                .Where(pair => pair.target.Table == type.Name && pair.target.Column.Length > 0))
            .Select(pair => new NestedSelectRelation
            {
                Name = Plural.Map(Entity(pair.child.Name)),
                Kind = NestedSelectRelationKind.List,
                From = key,
                To = Entity(pair.field.Name),
                Target = Build(pair.child, catalog, next),
            });
        return new NestedSelectType
        {
            Name = Entity(type.Name),
            Key = key,
            Properties = properties,
            Relations = [.. objects, .. lists],
        };
    }

    static (string Table, string Column) Target(SpecTypeField field)
    {
        if (field.ReferencePair is { Count: 2 } pair) return (pair[0], pair[1]);
        var parts = (field.References ?? "").Split('.', 2);
        return (parts.ElementAtOrDefault(0) ?? "", parts.ElementAtOrDefault(1) ?? "");
    }

    static string RelationName(string field, string table)
    {
        var entity = Entity(field);
        return entity.EndsWith("Id", StringComparison.Ordinal) && entity.Length > 2 ? entity[..^2] : Entity(table);
    }

    static string Entity(string name) => Pascal.Map(name);
}
