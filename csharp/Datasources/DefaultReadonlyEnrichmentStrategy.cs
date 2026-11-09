namespace Datasources;

public class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
{
    public virtual Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        enrichment.Type switch
        {
            EnrichmentPropertyType.List => ApplyList(via, enrichment, parents, sources),
            EnrichmentPropertyType.Object => ApplyObject(via, enrichment, parents, sources),
            EnrichmentPropertyType.Dictionary => ApplyDictionary(via, enrichment, parents, sources),
            _ => throw new DatasourceException($"Unsupported enrichment type \"{enrichment.Type}\"")
        };

    public virtual async Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TLink> link,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], enrichment, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var row in link.Read(new Where(new Operator.Equals(association.Parent, Value.From(key)))))
            {
                if (Get(row!, association.Related) is not { } id) continue;
                var entity = await via.ReadSingle(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(id))));
                if (entity is not null)
                    related.Add(entity);
            }
            Set(parents[i], enrichment.DestinationProperty, related);
        }
    }

    protected virtual async Task ApplyObject<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var cache = new Dictionary<string, TRelated?>();
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            var cacheKey = KeyOf(key);
            if (!cache.TryGetValue(cacheKey, out var related))
            {
                related = await via.ReadSingle(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key))));
                cache[cacheKey] = related;
            }
            if (related is null) continue;
            Set(parent, enrichment.DestinationProperty, enrichment.RelatedProperty is { } select ? Get(related, select) : related);
        }
    }

    protected virtual async Task ApplyList<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var item in via.Read(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key)))))
                related.Add(item);
            Set(parent, enrichment.DestinationProperty, related);
        }
    }

    protected virtual async Task ApplyDictionary<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var schema = via.Schema;
        var mapKey = schema.DictionaryKey;
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            var map = EnrichmentCells.MapOf(parent, enrichment.DestinationProperty);
            await foreach (var item in via.Read(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key)))))
            {
                if (Get(item!, mapKey) is not { } entry) continue;
                map[entry] = EnrichmentCells.DictionaryValue(item, enrichment, schema, map);
            }
            Set(parent, enrichment.DestinationProperty, map);
        }
    }

    protected static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;

    protected static object? SourceKey<TParent>(TParent parent, Enrichment enrichment, Row? source)
    {
        var name = enrichment.SourceProperty;
        return source is not null && source.TryGetValue(name, out var cell) && cell is not Value.Null
            ? cell.ToObject()
            : Get(parent, name);
    }

    protected static string KeyOf(object? key) => key?.ToString() ?? "";

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    protected static void Set<TItem>(TItem obj, string name, object? value) =>
        EnrichmentCells.Set(obj, name, value);
}
