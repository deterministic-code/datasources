namespace Datasources.CodeGenerators;

public static class ByGroups
{
    public static Dictionary<string, IReadOnlyList<IReadOnlyList<string>>> From(IReadOnlyList<SqlTable> types) =>
        types.ToDictionary(type => type.Name, type => Of(type), StringComparer.Ordinal);

    static IReadOnlyList<IReadOnlyList<string>> Of(SqlTable type)
    {
        var overlay = SqlSchema.OverlayOf(type);
        var pk = SpecTables.PkName(type.AsType(), overlay);
        var groups = new List<IReadOnlyList<string>>();
        if (pk is not null) groups.Add([pk]);
        void Add(IReadOnlyList<string> props)
        {
            if (props.Count == 0) return;
            if (groups.Any(group => group.SequenceEqual(props))) return;
            groups.Add(props);
        }
        foreach (var name in SpecTables.UniqueLookupFields(type.AsType(), overlay))
            if (name != pk) Add([name]);
        foreach (var index in type.Indexes.Where(index => index.IsUnique))
            Add(index.Fields);
        return groups;
    }
}
