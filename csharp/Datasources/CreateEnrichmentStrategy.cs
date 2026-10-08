using System.Collections;
using System.Reflection;

namespace Datasources;

public abstract class CreateEnrichmentStrategy<T>
{
    public abstract Task CreateOwned(IChildPersist writer, Enrichment enrichment, object parent);

    public abstract Task CreateLinked(IChildPersist writer, Enrichment enrichment, object parent);

    protected static List<object> RelatedIds(object parent, Enrichment enrichment)
    {
        var ids = new List<object>();
        var seen = new HashSet<string>();
        foreach (var related in Payload(parent, enrichment.DestinationProperty))
        {
            if (Get(related, enrichment.RelatedKey) is not object id || Missing(id))
                throw new DatasourceException($"{enrichment.DestinationProperty} requires \"{enrichment.RelatedKey}\"");
            if (seen.Add(id.ToString() ?? ""))
                ids.Add(id);
        }
        return ids;
    }

    protected static object Required(object parent, string name, string into) =>
        Get(parent, name) is { } key && !Missing(key)
            ? key
            : throw new DatasourceException($"{into} requires \"{name}\"");

    protected static bool Missing(object? value) => value is null or 0 or 0L or 0u or "";

    protected static IReadOnlyList<object> Payload(object parent, string into) =>
        Get(parent, into) switch
        {
            null or string => [],
            IEnumerable rows => [.. rows.Cast<object>().Where(item => item is not null and not string)],
            _ => []
        };

    protected static IReadOnlyList<object> Owned(IChildPersist writer, Enrichment enrichment, object parent) =>
        enrichment.Type is EnrichmentPropertyType.Dictionary
            ? DictionaryOwned(writer, enrichment, parent)
            : Payload(parent, enrichment.DestinationProperty);

    static List<object> DictionaryOwned(IChildPersist writer, Enrichment enrichment, object parent)
    {
        if (Get(parent, enrichment.DestinationProperty) is not IDictionary map) return [];
        var rows = new List<object>();
        foreach (DictionaryEntry entry in map)
        {
            if (entry.Key is null) continue;
            var row = writer.New();
            Set(row, MapKey(enrichment.RelatedKey), entry.Key);
            if (enrichment.RelatedProperty is { } select)
                Set(row, select, entry.Value);
            else if (entry.Value is not null)
                foreach (var prop in entry.Value.GetType().GetProperties())
                    Set(row, prop.Name, prop.GetValue(entry.Value));
            rows.Add(row);
        }
        return rows;
    }

    static string MapKey(string relatedKey) =>
        relatedKey.Contains('_') || relatedKey.Length > 0 && char.IsLower(relatedKey[0]) ? "key" : "Key";

    protected static void Stamp(object related, string by, object? key)
    {
        if (key is not null)
            Set(related, by, key);
    }

    protected static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    protected static void Set(object obj, string name, object? value)
    {
        var prop = Prop(obj, name);
        var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        if (value is not null && IsReference(target) && !target.IsInstanceOfType(value))
        {
            var current = prop.GetValue(obj) ?? Activator.CreateInstance(target)
                ?? throw new DatasourceException($"Cannot create \"{name}\"");
            var id = target.GetProperty("Id") ?? target.GetProperty("id")
                ?? throw new DatasourceException($"\"{name}\" requires Id");
            var idType = Nullable.GetUnderlyingType(id.PropertyType) ?? id.PropertyType;
            id.SetValue(current, idType.IsInstanceOfType(value) ? value : Convert.ChangeType(value, idType));
            prop.SetValue(obj, current);
            return;
        }
        prop.SetValue(obj, value is null || target.IsInstanceOfType(value)
            ? value
            : Convert.ChangeType(value, target));
    }

    static bool IsReference(Type type) =>
        type.IsClass && type != typeof(string) && !typeof(IEnumerable).IsAssignableFrom(type);

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
