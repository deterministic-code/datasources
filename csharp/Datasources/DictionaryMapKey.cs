namespace Datasources;

internal static class DictionaryMapKey
{
    public static string Of<TChild>(Enrichment enrichment, Type? valueType = null)
    {
        var skip = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "Id", enrichment.RelatedKey };
        if (enrichment.RelatedProperty is { } related)
            skip.Add(related);
        if (valueType is { } dest && dest != typeof(string) && dest != typeof(object) && !dest.IsPrimitive && dest != typeof(Value))
            foreach (var prop in dest.GetProperties())
                skip.Add(prop.Name);
        return typeof(TChild).GetProperties()
            .Select(prop => prop.Name)
            .Where(name => !skip.Contains(name))
            .Single();
    }

    public static Type? ValueType(Type parent, string destination)
    {
        var dest = parent.GetProperty(destination)?.PropertyType;
        return dest is { IsGenericType: true } && dest.GetGenericArguments().Length == 2
            ? dest.GetGenericArguments()[1]
            : null;
    }
}
