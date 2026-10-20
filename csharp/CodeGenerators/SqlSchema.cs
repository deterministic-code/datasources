namespace Datasources.CodeGenerators;

public static class SqlSchema
{
    public static bool UseStoredProcedures(IReadOnlyDictionary<string, string> settings) =>
        settings.TryGetValue("datasource.use_stored_procedures", out var value) && value == "true";

    public static DatasourceTable OverlayOf(SqlTable table) => new()
    {
        Name = table.Name,
        Mapping = table.Mapping,
        Fields = table.Overlays,
        Indexes = table.Indexes,
        UniqueIndexFields = table.UniqueIndexFields,
    };

    public static DatasourceFieldOverlay? FieldOverlay(SqlTable table, string fieldName) =>
        table.Overlays.FirstOrDefault(overlay => overlay.Name == fieldName);

    public static IReadOnlyList<string> TableIdentity(SqlTable table)
    {
        var keys = SpecTables.IdentityColumns(table.AsType(), OverlayOf(table));
        if (keys.Count > 1)
            return keys.All(col => table.Fields.Any(f => f.Name == col)) ? keys : [];
        var col = keys.FirstOrDefault();
        if (col is null) return [];
        return table.Fields.Any(f => f.Name == col) || table.Inherits == "set" ? [col] : [];
    }

    public static bool IsGeneratedIdentity(SqlTable table)
    {
        var keys = TableIdentity(table);
        return keys.Count == 1 && FieldOverlay(table, keys[0])?.IsFixedId != true;
    }

    public static bool UsesGeneratedIdColumn(SqlTable table, SpecTypeField field) =>
        field.IsId && IsGeneratedIdentity(table);

    public static IReadOnlyList<SqlTable> From(ParsedSpec spec)
    {
        foreach (var type in spec.Types)
            foreach (var field in type.Fields)
                if (field.Name == "id" && !field.IsId)
                    throw new InvalidOperationException($"{type.Name}.id must have is_id: true");
        var overlays = SpecTables.TableByName(spec);
        var byName = spec.ExpandedTypes.ToDictionary(t => t.Name, StringComparer.Ordinal);
        return SpecTables.DatasourceTypesOf(spec).Select(type =>
        {
            overlays.TryGetValue(type.Name, out var table);
            var uniqueCols = SpecTables.DictionaryUniqueColumns(type, byName);
            var indexes = (table?.Indexes ?? []).ToList();
            if (uniqueCols.Count > 0 && !indexes.Any(idx =>
                idx.IsUnique && idx.Fields.Count == uniqueCols.Count && idx.Fields.SequenceEqual(uniqueCols)))
            {
                indexes.Add(new DatasourceIndex { Name = "owner_key", Fields = uniqueCols, IsUnique = true });
            }
            return new SqlTable
            {
                Name = type.Name,
                Tags = type.Tags,
                Kind = type.Kind,
                Inherits = type.Inherits,
                Ids = type.Ids,
                Fields = SpecTables.PersistedColumnFields(type, byName),
                Indexes = indexes,
                UniqueIndexFields = table?.UniqueIndexFields ?? [],
                Mapping = table?.Mapping,
                Overlays = table?.Fields ?? [],
            };
        }).ToList();
    }

    public static bool HasAuditColumns(SqlTable table) =>
        table.Fields.Any(f => f.Name == "created") && table.Fields.Any(f => f.Name == "updated");

    public static IReadOnlyList<SqlTable> BuildLive(IReadOnlyList<SqlTable> types, SqlMapping mapping) =>
        Topo(types.Select(t => t with { TableName = mapping.TableName(t) }).ToList());

    static List<SqlTable> Topo(List<SqlTable> tables)
    {
        var names = tables.Select(t => t.Name).ToHashSet(StringComparer.Ordinal);
        var deps = tables.ToDictionary(
            t => t.Name,
            t => t.Fields
                .Select(f => SpecTables.ReferenceTarget(f).Table)
                .Where(dep => names.Contains(dep) && dep != t.Name && !string.IsNullOrEmpty(dep))
                .ToHashSet(StringComparer.Ordinal));
        var output = new List<SqlTable>();
        var done = new HashSet<string>(StringComparer.Ordinal);
        var pending = tables.ToList();
        while (pending.Count > 0)
        {
            var idx = pending.FindIndex(t => deps[t.Name].All(done.Contains));
            if (idx == -1)
            {
                output.AddRange(pending);
                break;
            }
            var t = pending[idx];
            pending.RemoveAt(idx);
            output.Add(t);
            done.Add(t.Name);
        }
        return output;
    }
}
