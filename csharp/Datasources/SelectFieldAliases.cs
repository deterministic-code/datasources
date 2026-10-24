namespace Datasources;

public static class SelectFieldAliases
{
    public static IReadOnlyList<string?> Of(
        string table,
        IReadOnlyList<string> fields,
        IReadOnlyList<string?>? aliases = null)
    {
        var result = new string?[fields.Count];
        var used = new HashSet<string>(StringComparer.Ordinal);
        if (aliases is { } provided)
        {
            for (var i = 0; i < fields.Count && i < provided.Count; i++)
            {
                if (provided[i] is { } alias)
                {
                    result[i] = alias;
                    used.Add(alias);
                }
            }
        }
        for (var i = 0; i < fields.Count; i++)
        {
            if (result[i] is null && PathOf(fields[i], table) is { Root: true } path)
            {
                result[i] = path.Last;
                used.Add(path.Last);
            }
        }
        for (var i = 0; i < fields.Count; i++)
        {
            if (result[i] is null && PathOf(fields[i], table) is { Root: false } path)
            {
                var alias = used.Contains(path.Last) ? $"{path.Owner}{path.Last}" : path.Last;
                result[i] = alias;
                used.Add(alias);
            }
        }
        return result;
    }

    public static string Apply(string field, string? alias) =>
        alias is { } name ? $"{field} AS {name}" : field;

    public static PathParts? PathOf(string field, string table)
    {
        if (AsIndex(field) >= 0 || field.Contains('('))
            return null;
        var parts = Segments(field);
        return parts.Count < 2 ? null : new(parts[^2], parts[^1], parts.Count == 2 && parts[0] == table);
    }

    public static (string Name, string Inner)? FunctionOf(string field)
    {
        var open = field.IndexOf('(');
        return open <= 0 || field[^1] != ')'
            ? null
            : (field[..open].Trim(), field[(open + 1)..^1].Trim());
    }

    public static int AsIndex(string field) => field.IndexOf(" AS ", StringComparison.OrdinalIgnoreCase);

    public static IReadOnlyList<string> Segments(string name)
    {
        var parts = new List<string>();
        var quoted = false;
        var start = 0;
        for (var i = 0; i < name.Length; i++)
        {
            if (name[i] == '"') quoted = !quoted;
            if (name[i] == '.' && !quoted)
            {
                parts.Add(name[start..i]);
                start = i + 1;
            }
        }
        parts.Add(name[start..]);
        return parts;
    }

    public readonly record struct PathParts(string Owner, string Last, bool Root);
}
