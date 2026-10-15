namespace Datasources.CodeGenerators;

public static class ByFields
{
    public static Dictionary<string, IReadOnlyList<string>> From(IReadOnlyList<SqlTable> types) =>
        types
            .Select(type =>
            {
                var overlay = SqlSchema.OverlayOf(type);
                var pk = SpecTables.PkName(type.AsType(), overlay);
                var names = SpecTables.UniqueLookupFields(type.AsType(), overlay).Where(name => name != pk).ToList();
                return (type.Name, names);
            })
            .Where(entry => entry.names.Count > 0)
            .ToDictionary(entry => entry.Name, entry => (IReadOnlyList<string>)entry.names, StringComparer.Ordinal);
}
