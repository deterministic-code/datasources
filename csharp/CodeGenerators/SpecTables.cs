namespace Datasources.CodeGenerators;

public static class SpecTables
{
    public static bool TypeHasTag(SpecType type, string tag) => type.Tags.Contains(tag);

    public static IReadOnlyList<SpecType> DatasourceTypesOf(ParsedSpec spec) =>
        spec.ExpandedTypes.Where(type => TypeHasTag(type, "datasource_type")).ToList();

    public static bool IsDictionaryType(SpecType? type) => type?.Inherits == "dictionary";

    public static bool IsCollectionField(SpecTypeField field) =>
        field.Kind == "type" && (field.IsArray || field.IsMap);

    public static IReadOnlyList<SpecTypeField> ColumnFields(IEnumerable<SpecTypeField> fields) =>
        fields.Where(field => !IsCollectionField(field)).ToList();

    public static bool HasAuthoredIdentity(SpecType type) =>
        type.Ids is { Count: > 0 } || type.Fields.Any(f => f.IsId);

    public static IReadOnlyList<string> IdentityColumns(SpecType? type, DatasourceTable? table = null)
    {
        if (type?.Ids is { Count: > 0 } ids) return ids;
        var marked = type?.Fields.Where(f => f.IsId).Select(f => f.Name).ToList() ?? [];
        if (marked.Count > 0) return marked;
        var fixedId = table?.Fields.FirstOrDefault(f => f.IsFixedId == true);
        if (fixedId is not null) return [fixedId.Name];
        return type?.Inherits == "set" || type is null ? ["id"] : [];
    }

    public static string PkName(SpecType type, DatasourceTable? table) =>
        IdentityColumns(type, table).FirstOrDefault() ?? "id";

    public static bool IsPkField(SpecTypeField field, SpecType type, DatasourceTable? table) =>
        IdentityColumns(type, table).Contains(field.Name);

    public static IReadOnlyList<string> UniqueLookupFields(SpecType type, DatasourceTable? table)
    {
        var names = new List<string>();
        void Add(string name)
        {
            if (names.Contains(name)) return;
            var field = type.Fields.FirstOrDefault(f => f.Name == name);
            if (field is not null && IsCollectionField(field)) return;
            names.Add(name);
        }
        foreach (var name in IdentityColumns(type, table)) Add(name);
        foreach (var overlay in table?.Fields ?? [])
            if (overlay.IsUnique == true) Add(overlay.Name);
        foreach (var name in table?.UniqueIndexFields ?? []) Add(name);
        return names;
    }

    public static Dictionary<string, DatasourceTable> TableByName(ParsedSpec spec) =>
        spec.Datasource.ToDictionary(table => table.Name, StringComparer.Ordinal);

    public static (string Table, string Column) ReferenceTarget(SpecTypeField field)
    {
        if (field.ReferencePair is { Count: 2 } pair) return (pair[0], pair[1]);
        var parts = (field.References ?? "").Split('.', 2);
        return (parts.ElementAtOrDefault(0) ?? "", parts.ElementAtOrDefault(1) ?? "");
    }

    public static (string Left, string Right)? SplitDot(string value)
    {
        var i = value.IndexOf('.');
        return i == -1 ? null : (Left: value[..i], Right: value[(i + 1)..]);
    }

    public static bool IsOwnerIdentityRef(SpecTypeField field, string selfName, IReadOnlyDictionary<string, SpecType> byName)
    {
        var refs = field.ReferencePair ?? (field.References is { } one ? [one] : Array.Empty<string>());
        if (refs.Count == 0) return false;
        var parts = refs.Select(r => SplitDot(r) ?? (Left: "", Right: r)).ToList();
        var owner = parts[0].Left;
        if (string.IsNullOrEmpty(owner) || owner == selfName) return false;
        var identity = IdentityNamesOf(byName.GetValueOrDefault(owner), byName);
        return parts.All(p => p.Left == owner && identity.Contains(p.Right));
    }

    static HashSet<string> IdentityNamesOf(SpecType? type, IReadOnlyDictionary<string, SpecType> byName, HashSet<string>? stack = null)
    {
        stack ??= [];
        if (type is null || !stack.Add(type.Name)) return [];
        var local = IdentityColumns(type);
        if (local.Count > 0) return [.. local];
        return type.Inherits is { } parent && parent != "dictionary"
            ? IdentityNamesOf(byName.GetValueOrDefault(parent), byName, stack)
            : [];
    }

    public static IReadOnlyList<SpecTypeField> PersistedColumnFields(SpecType type, IReadOnlyDictionary<string, SpecType>? byName = null) =>
        ColumnFields(type.Fields);

    static string RoleColumn(SpecType dict, string role, string tableName)
    {
        var field = dict.Fields.FirstOrDefault(item => item.Name == role);
        if (field?.References is { } references && SplitDot(references) is { } split && split.Left == tableName)
            return split.Right;
        return role;
    }

    public static IReadOnlyList<string> DictionaryUniqueColumns(SpecType type, IReadOnlyDictionary<string, SpecType> byName)
    {
        foreach (var candidate in byName.Values)
        {
            foreach (var field in candidate.Fields)
            {
                if (!field.IsMap) continue;
                if (!byName.TryGetValue(field.Base, out var dict) || !IsDictionaryType(dict)) continue;
                if (field.References is not { } references) continue;
                var split = SplitDot(references);
                if (split is null || split.Value.Left != type.Name) continue;
                return [split.Value.Right, RoleColumn(dict, "key", type.Name)];
            }
        }
        return [];
    }
}
