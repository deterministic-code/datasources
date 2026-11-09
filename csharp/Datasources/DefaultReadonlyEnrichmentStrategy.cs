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
            var dest = Prop(parent, enrichment.DestinationProperty);
            var map = MapOf(parent, dest);
            await foreach (var item in via.Read(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key)))))
            {
                if (Get(item!, mapKey) is not { } entry) continue;
                map[entry] = DictionaryValue(item, enrichment, schema, dest.PropertyType);
            }
            Set(parent, enrichment.DestinationProperty, map);
        }
    }

    static IDictionary MapOf<TParent>(TParent parent, PropertyInfo dest)
    {
        if (Get(parent, dest.Name) is IDictionary existing)
        {
            existing.Clear();
            return existing;
        }
        return (IDictionary)(Activator.CreateInstance(dest.PropertyType)
            ?? throw new DatasourceException($"Cannot create \"{dest.Name}\""));
    }

    static object? DictionaryValue<TRelated>(TRelated item, Enrichment enrichment, Schema schema, Type mapType)
    {
        if (enrichment.RelatedProperty is { } select)
            return Get(item, select);
        var cells = new Row();
        foreach (var name in schema.DictionaryValueColumns(enrichment.RelatedKey, schema.DictionaryKey))
            if (item!.GetType().GetProperty(name)?.GetValue(item) is { } value)
                cells[name] = Value.From(value);
        var valueType = mapType.IsGenericType ? mapType.GetGenericArguments()[1] : typeof(object);
        return valueType == typeof(string) || valueType.IsValueType
            ? cells.Values.Select(cell => cell.ToObject()).FirstOrDefault()
            : Converted(valueType, cells);
    }

    static object Converted(Type type, Row cells)
    {
        var converterType = typeof(MetaDataRowTypeConverter<>).MakeGenericType(type);
        var converter = Activator.CreateInstance(converterType, [null])
            ?? throw new DatasourceException($"Cannot create \"{type.Name}\"");
        return converterType.GetMethod(nameof(MetaDataRowTypeConverter<Row>.Convert), [typeof(Row)])!
            .Invoke(converter, [cells])!;
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

    protected static object? Get<TItem>(TItem obj, string name) =>
        obj is IDictionary dict
            ? dict.Contains(name) ? dict[name] : null
            : obj!.GetType().GetProperty(name)?.GetValue(obj);

    protected static void Set<TItem>(TItem obj, string name, object? value) => Prop(obj, name).SetValue(obj, value);

    static PropertyInfo Prop<TItem>(TItem obj, string name) =>
        obj!.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
