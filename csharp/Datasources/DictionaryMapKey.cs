namespace Datasources;

internal static class DictionaryMapKey
{
    public static string Of(Schema schema, Enrichment enrichment, IEnumerable<string>? extraSkip = null)
    {
        var skip = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "Id", enrichment.RelatedKey };
        if (enrichment.RelatedProperty is { } related)
            skip.Add(related);
        if (extraSkip is not null)
            foreach (var name in extraSkip)
                skip.Add(name);
        return schema.Columns
            .Where(pair => !skip.Contains(pair.Key) && !pair.Value.HasDefault)
            .Select(pair => pair.Key)
            .FirstOrDefault()
            ?? throw new DatasourceException("dictionary map key is missing on the related object");
    }
}
