using System.Collections;
using System.Reflection;

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
            var key = SourceKey(parents[i], enrichment.SourceProperty, At(sources, i));
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
            var key = SourceKey(parent, enrichment.SourceProperty, At(sources, i));
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
            var key = SourceKey(parent, enrichment.SourceProperty, At(sources, i));
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
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment.SourceProperty, At(sources, i));
            if (key is null) continue;
            var map = (IDictionary)Activator.CreateInstance(Prop(parent, enrichment.DestinationProperty).PropertyType)!;
            await foreach (var item in via.Read(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key)))))
            {
                if (Get(item!, MapKey(enrichment.RelatedKey)) is not { } entry) continue;
                map[entry] = enrichment.RelatedProperty is { } select
                    ? Get(item, select)
                    : CopyValue(item!, enrichment, Prop(parent, enrichment.DestinationProperty).PropertyType);
            }
            Set(parent, enrichment.DestinationProperty, map);
        }
    }

    static string MapKey(string relatedKey) =>
        relatedKey.Contains('_') || relatedKey.Length > 0 && char.IsLower(relatedKey[0]) ? "key" : "Key";

    static object? CopyValue(object item, Enrichment enrichment, Type mapType)
    {
        var valueType = mapType.IsGenericType ? mapType.GetGenericArguments()[1] : typeof(object);
        if (valueType == typeof(string) || valueType.IsValueType)
            return Get(item, "Value") ?? Get(item, "value");
        var copy = Activator.CreateInstance(valueType)!;
        var skip = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { MapKey(enrichment.RelatedKey), enrichment.RelatedKey };
        foreach (var prop in item.GetType().GetProperties())
        {
            if (skip.Contains(prop.Name)) continue;
            valueType.GetProperty(prop.Name)?.SetValue(copy, prop.GetValue(item));
        }
        return copy;
    }

    protected static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;

    protected static object? SourceKey<TParent>(TParent parent, string name, Row? source) =>
        LookupKey(
            source is not null && source.TryGetValue(name, out var cell) && cell is not Value.Null
                ? cell.ToObject()
                : parent!.GetType().GetProperty(name)?.GetValue(parent));

    static object? LookupKey(object? value)
    {
        if (value is null || value is string || value.GetType().IsPrimitive || value is Value) return value;
        return value.GetType().GetProperty("Id")?.GetValue(value)
            ?? value.GetType().GetProperty("id")?.GetValue(value)
            ?? value;
    }

    protected static string KeyOf(object? key) => key?.ToString() ?? "";

    protected static object? Get<TItem>(TItem obj, string name) => Prop(obj, name).GetValue(obj);

    protected static void Set<TItem>(TItem obj, string name, object? value) => Prop(obj, name).SetValue(obj, value);

    static PropertyInfo Prop<TItem>(TItem obj, string name) =>
        obj!.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
