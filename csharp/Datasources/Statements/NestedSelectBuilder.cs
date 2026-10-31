namespace Datasources;

public sealed class NestedSelectBuilder
{
    public NestedSelectStatement Build(NestedSelectType root, NestedSelectBuilderOptions? options = null)
    {
        var chosen = options ?? new NestedSelectBuilderOptions();
        var fields = new List<string>();
        var joins = new List<Join>();
        Walk(root, root.Name, root.Name, "", chosen.StopAt, false, fields, joins);
        var aliases = SelectFieldAliases.Of(root.Name, fields);
        var byPath = fields
            .Select((field, i) => (field, alias: aliases[i] ?? field))
            .ToDictionary(pair => pair.field, pair => pair.alias, StringComparer.Ordinal);
        return new NestedSelectStatement
        {
            Select = new SelectStatement(new()
            {
                Table = root.Name,
                Fields = fields,
                Joins = joins,
                Where = chosen.Where,
            }),
            Map = MapOf(root, root.Name, "", chosen.StopAt, byPath),
        };
    }

    static void Walk(
        NestedSelectType type,
        string fieldPrefix,
        string joinEntity,
        string stopPrefix,
        IReadOnlyList<string> stopAt,
        bool outer,
        List<string> fields,
        List<Join> joins)
    {
        foreach (var property in type.Properties)
            fields.Add($"{fieldPrefix}.{property}");
        foreach (var relation in type.Relations)
        {
            var path = stopPrefix.Length == 0 ? relation.Name : $"{stopPrefix}.{relation.Name}";
            if (Stopped(stopAt, path)) continue;
            var list = relation.Kind == NestedSelectRelationKind.List;
            joins.Add(list || outer
                ? Join.Left(relation.Target.Name, $"{joinEntity}.{relation.From}", $"{relation.Target.Name}.{relation.To}")
                : Join.Inner(relation.Target.Name, $"{joinEntity}.{relation.From}", $"{relation.Target.Name}.{relation.To}"));
            Walk(relation.Target, $"{fieldPrefix}.{relation.Target.Name}", relation.Target.Name, path, stopAt, outer || list, fields, joins);
        }
    }

    static TypeMap MapOf(
        NestedSelectType type,
        string fieldPrefix,
        string stopPrefix,
        IReadOnlyList<string> stopAt,
        IReadOnlyDictionary<string, string> aliases) =>
        new()
        {
            Key = aliases[$"{fieldPrefix}.{type.Key}"],
            Fields = FieldsOf(type, fieldPrefix, stopPrefix, stopAt, aliases),
        };

    static Dictionary<string, TypeMapField> FieldsOf(
        NestedSelectType type,
        string fieldPrefix,
        string stopPrefix,
        IReadOnlyList<string> stopAt,
        IReadOnlyDictionary<string, string> aliases)
    {
        var fields = new Dictionary<string, TypeMapField>(StringComparer.Ordinal);
        foreach (var property in type.Properties)
        {
            var alias = aliases[$"{fieldPrefix}.{property}"];
            fields[alias] = alias;
        }
        foreach (var relation in type.Relations)
        {
            var path = stopPrefix.Length == 0 ? relation.Name : $"{stopPrefix}.{relation.Name}";
            if (Stopped(stopAt, path)) continue;
            var childPrefix = $"{fieldPrefix}.{relation.Target.Name}";
            if (relation.Kind == NestedSelectRelationKind.List)
            {
                fields[relation.Name] = new TypeMapGroup
                {
                    Key = aliases[$"{childPrefix}.{relation.Target.Key}"],
                    Fields = FieldsOf(relation.Target, childPrefix, path, stopAt, aliases),
                };
                continue;
            }
            foreach (var pair in FieldsOf(relation.Target, childPrefix, path, stopAt, aliases))
                fields[pair.Key] = pair.Value;
        }
        return fields;
    }

    static bool Stopped(IReadOnlyList<string> stopAt, string path) =>
        stopAt.Any(item => item == path || path.StartsWith($"{item}.", StringComparison.Ordinal));
}
