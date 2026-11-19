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
            var cacheKey = MatchKey(parent, enrichment, At(sources, i));
            if (cacheKey is null) continue;
            if (!cache.TryGetValue(cacheKey, out var related))
            {
                var clause = MatchWhere(parent, enrichment, At(sources, i));
                if (clause is null) continue;
                related = await via.ReadSingle(clause);
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
            var clause = MatchWhere(parent, enrichment, At(sources, i));
            if (clause is null) continue;
            var related = new List<TRelated>();
            await foreach (var item in via.Read(clause))
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
        var mapKey = EnrichmentCells.MapKey(enrichment);
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var clause = MatchWhere(parent, enrichment, At(sources, i));
            if (clause is null) continue;
            var map = EnrichmentCells.MapOf(parent, enrichment.DestinationProperty);
            await foreach (var item in via.Read(clause))
            {
                if (Get(item!, mapKey) is not { } entry) continue;
                map[entry] = EnrichmentCells.DictionaryValue(item, enrichment, schema, map);
            }
            Set(parent, enrichment.DestinationProperty, map);
        }
    }

    internal static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;

    protected static object? SourceValue<TParent>(TParent parent, string name, Row? source) =>
        source is not null && source.TryGetValue(name, out var cell) && cell is not Value.Null
            ? cell.ToObject()
            : Get(parent, name);

    protected static object? SourceKey<TParent>(TParent parent, Enrichment enrichment, Row? source) =>
        SourceValue(parent, enrichment.SourceProperty, source);

    internal static string? MatchKey<TParent>(TParent parent, Enrichment enrichment, Row? source)
    {
        var parts = new List<string>();
        foreach (var pair in enrichment.Pairs())
        {
            var key = SourceValue(parent, pair.SourceProperty, source);
            if (key is null) return null;
            parts.Add(KeyOf(key));
        }
        return string.Join('\0', parts);
    }

    internal static string? RowMatchKey<TItem>(TItem row, Enrichment enrichment)
    {
        var parts = new List<string>();
        foreach (var pair in enrichment.Pairs())
        {
            var key = Get(row, pair.RelatedKey);
            if (key is null) return null;
            parts.Add(KeyOf(key));
        }
        return string.Join('\0', parts);
    }

    protected static Where? MatchWhere<TParent>(TParent parent, Enrichment enrichment, Row? source)
    {
        Condition? condition = null;
        foreach (var pair in enrichment.Pairs())
        {
            var key = SourceValue(parent, pair.SourceProperty, source);
            if (key is null) return null;
            Condition equals = new Operator.Equals(pair.RelatedKey, Value.From(key));
            condition = condition is null ? equals : new Operator.And(condition, equals);
        }
        return condition is null ? null : new Where(condition);
    }

    internal static string KeyOf(object? key) => key?.ToString() ?? "";

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    protected static void Set<TItem>(TItem obj, string name, object? value) =>
        EnrichmentCells.Set(obj, name, value);
}
