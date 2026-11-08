using System.Collections;

namespace Datasources;

public abstract class UpdateEnrichmentStrategy<T>
{
    public abstract Task UpdateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    public abstract Task UpdateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    protected static List<Value> RelatedIds<TParent>(TParent parent, Enrichment enrichment)
    {
        var ids = new List<Value>();
        var seen = new HashSet<string>();
        foreach (var related in Payload<TParent, object>(parent, enrichment.DestinationProperty))
        {
            var id = Value.From(Get(related, enrichment.RelatedKey));
            if (Missing(id))
                throw new DatasourceException($"{enrichment.DestinationProperty} requires \"{enrichment.RelatedKey}\"");
            if (seen.Add(Text(id)))
                ids.Add(id);
        }
        return ids;
    }

    protected static Value Required<TParent>(TParent parent, string name, string into) =>
        Value.From(Get(parent, name)) is var key && !Missing(key)
            ? key
            : throw new DatasourceException($"{into} requires \"{name}\"");

    protected static bool Missing(Value? value) => value is null or Value.Null or Value.Int(0) or Value.Text("");

    protected static string Text(Value value) => value.ToObject()?.ToString() ?? "";

    protected static IReadOnlyList<TItem> Payload<TParent, TItem>(TParent parent, string into) =>
        Get(parent, into) switch
        {
            null or string => [],
            IEnumerable rows => [.. rows.OfType<TItem>().Where(item => item is not string)],
            _ => []
        };

    protected static IReadOnlyList<TChild> Children<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
        where TChild : new() =>
        enrichment.Type is EnrichmentPropertyType.Dictionary
            ? DictionaryChildren(writer, enrichment, parent)
            : Payload<TParent, TChild>(parent, enrichment.DestinationProperty);

    static List<TChild> DictionaryChildren<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
        where TChild : new()
    {
        if (Get(parent, enrichment.DestinationProperty) is not IDictionary map) return [];
        var keyName = DictionaryMapKey.Of(writer.Schema, enrichment, NamesOf(map));
        var rows = new List<TChild>();
        foreach (DictionaryEntry entry in map)
        {
            if (entry.Key is null) continue;
            var row = new TChild();
            Set(row, keyName, entry.Key);
            if (enrichment.RelatedProperty is { } select)
                Set(row, select, entry.Value);
            else if (entry.Value is not null)
                MetaDataRowTypeConverter.Copy(entry.Value, row);
            rows.Add(row);
        }
        return [.. rows.OrderBy(row => Get(row, keyName)?.ToString() ?? "", StringComparer.Ordinal)];
    }

    static IEnumerable<string> NamesOf(IDictionary map)
    {
        foreach (DictionaryEntry entry in map)
            if (entry.Value is not null and not string)
                foreach (var name in MetaDataRowTypeConverter.Names(entry.Value))
                    yield return name;
    }

    protected static void Stamp<TItem>(TItem related, string by, object? key)
    {
        if (key is not null)
            Set(related, by, key);
    }

    protected static bool HasKey<TItem>(TItem item, IReadOnlyList<string> keys) =>
        keys.Count > 0 && keys.All(name => Get(item, name) is not null and not 0 and not 0L and not 0u and not "");

    protected static string KeyOf<TItem>(TItem item, IReadOnlyList<string> keys) =>
        string.Join("\u001f", keys.Select(name => Get(item, name)?.ToString() ?? ""));

    protected static object? Get<TItem>(TItem obj, string name) =>
        obj is IDictionary || MetaDataRowTypeConverter.Has(obj, name)
            ? MetaDataRowTypeConverter.Get(obj, name)
            : throw new DatasourceException($"Unknown property \"{name}\"");

    protected static void Set<TItem>(TItem obj, string name, object? value)
    {
        if (!MetaDataRowTypeConverter.TrySet(obj!, name, value))
            throw new DatasourceException($"Unknown property \"{name}\"");
    }
}
