namespace Datasources.CodeGenerators;

public static class InferReferenceTypes
{
    public static IReadOnlyList<SpecType> Apply(IReadOnlyList<SpecType> types)
    {
        var working = types.Select(type => Copy(type)).ToList();
        for (var pass = 0; pass < working.Count; pass++)
        {
            var byName = working.ToDictionary(type => type.Name, StringComparer.Ordinal);
            var changed = false;
            for (var i = 0; i < working.Count; i++)
            {
                var fields = working[i].Fields.Select(field => Infer(field, byName, out var next) ? next : field).ToList();
                if (!fields.SequenceEqual(working[i].Fields))
                {
                    working[i] = Copy(working[i], fields);
                    changed = true;
                }
            }
            if (!changed) break;
        }
        return working;
    }

    static SpecType Copy(SpecType type, IReadOnlyList<SpecTypeField>? fields = null) => new()
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
        Fields = fields ?? [.. type.Fields],
    };

    static bool Infer(SpecTypeField field, IReadOnlyDictionary<string, SpecType> byName, out SpecTypeField inferred)
    {
        inferred = field;
        if (field.IsArray || field.IsMap || field.Type != "string" || field.Size is not null) return false;
        var (table, column) = SpecTables.ReferenceTarget(field);
        if (table == "" || column == "") return false;
        if (!byName.TryGetValue(table, out var parent) || parent.Fields.FirstOrDefault(f => f.Name == column) is not { } target)
            return false;
        if (target.Type == "string" && target.Size is null) return false;
        inferred = field with { Type = target.Type, Kind = target.Kind, Base = target.Base, Size = target.Size };
        return true;
    }
}
