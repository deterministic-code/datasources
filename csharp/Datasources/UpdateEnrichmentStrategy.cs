using System.Collections;
using System.Reflection;

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
            ? DictionaryChildren<TParent, TChild>(enrichment, parent)
            : Payload<TParent, TChild>(parent, enrichment.DestinationProperty);

    static List<TChild> DictionaryChildren<TParent, TChild>(Enrichment enrichment, TParent parent)
        where TChild : new()
    {
        if (Get(parent, enrichment.DestinationProperty) is not IDictionary map) return [];
        var rows = new List<TChild>();
        foreach (DictionaryEntry entry in map)
        {
            if (entry.Key is null) continue;
            var row = new TChild();
            Set(row, MapKey(enrichment.RelatedKey), entry.Key);
            if (enrichment.RelatedProperty is { } select)
                Set(row, select, entry.Value);
            else if (entry.Value is not null)
                foreach (var prop in entry.Value.GetType().GetProperties())
                    Set(row, prop.Name, prop.GetValue(entry.Value));
            rows.Add(row);
        }
        return [.. rows.OrderBy(row => Get(row, MapKey(enrichment.RelatedKey))?.ToString() ?? "", StringComparer.Ordinal)];
    }

    static string MapKey(string relatedKey) =>
        relatedKey.Contains('_') || relatedKey.Length > 0 && char.IsLower(relatedKey[0]) ? "key" : "Key";

    protected static void Stamp<TItem>(TItem related, string by, object? key)
    {
        if (key is not null)
            Set(related, by, key);
    }

    protected static bool HasKey<TItem>(TItem item, IReadOnlyList<string> keys) =>
        keys.Count > 0 && keys.All(name => Get(item, name) is not null and not 0 and not 0L and not 0u and not "");

    protected static string KeyOf<TItem>(TItem item, IReadOnlyList<string> keys) =>
        string.Join("\u001f", keys.Select(name => Get(item, name)?.ToString() ?? ""));

    protected static object? Get<TItem>(TItem obj, string name) => Prop(obj, name).GetValue(obj);

    protected static void Set<TItem>(TItem obj, string name, object? value)
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

    static PropertyInfo Prop<TItem>(TItem obj, string name) =>
        obj!.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
