using System.Collections;
using System.Reflection;

namespace Datasources;

static class EnrichmentCells
{
    internal static bool Missing(Value? value) => value is null or Value.Null or Value.Int(0) or Value.Text("");

    internal static string Text(Value value) => value.ToObject()?.ToString() ?? "";

    internal static object? Get<TItem>(TItem obj, string name) =>
        obj is IDictionary dict
            ? dict.Contains(name) ? dict[name] : null
            : obj!.GetType().GetProperty(name)?.GetValue(obj);

    internal static void Set<TItem>(TItem obj, string name, object? value)
    {
        if (obj is IDictionary dict)
        {
            dict[name] = value;
            return;
        }
        Write(Prop(obj, name), obj!, value);
    }

    internal static void Stamp<TItem>(TItem related, string by, object? key)
    {
        if (key is null)
            return;
        if (TryStamp(Get(related, by), by, key))
            return;
        Set(related, by, key);
    }

    internal static IReadOnlyList<TItem> Payload<TParent, TItem>(TParent parent, string into) =>
        Get(parent, into) switch
        {
            null or string => [],
            IEnumerable rows => [.. rows.OfType<TItem>().Where(item => item is not string)],
            _ => []
        };

    internal static IReadOnlyList<TChild> Children<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
        where TChild : new() =>
        enrichment.Type is EnrichmentPropertyType.Dictionary
            ? DictionaryChildren<TParent, TChild>(writer.Schema, enrichment, parent)
            : Payload<TParent, TChild>(parent, enrichment.DestinationProperty);

    internal static List<Value> RelatedIds<TParent>(TParent parent, Enrichment enrichment)
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

    internal static Value Required<TParent>(TParent parent, string name, string into) =>
        Value.From(Get(parent, name)) is var key && !Missing(key)
            ? key
            : throw new DatasourceException($"{into} requires \"{name}\"");

    internal static IDictionary MapOf<TParent>(TParent parent, string dest)
    {
        if (Get(parent, dest) is IDictionary existing)
        {
            existing.Clear();
            return existing;
        }
        throw new DatasourceException($"Cannot create \"{dest}\"");
    }

    internal static object? DictionaryValue<TRelated>(
        TRelated item,
        Enrichment enrichment,
        Schema schema,
        IDictionary map)
    {
        if (enrichment.RelatedProperty is { } select)
            return Get(item, select);
        var names = ValueColumns(schema, enrichment);
        var dest = NewMapValue(map) ?? (object?)item;
        foreach (var name in names)
            Copy(dest, name, Get(item, name));
        return dest;
    }

    static List<TChild> DictionaryChildren<TParent, TChild>(Schema schema, Enrichment enrichment, TParent parent)
        where TChild : new()
    {
        if (Get(parent, enrichment.DestinationProperty) is not IDictionary map) return [];
        var mapKey = MapKey(enrichment);
        var rows = new List<TChild>();
        foreach (DictionaryEntry entry in map)
        {
            if (entry.Key is null) continue;
            var row = new TChild();
            Set(row, mapKey, entry.Key);
            if (enrichment.RelatedProperty is { } select)
                Set(row, select, entry.Value);
            else if (entry.Value is not null)
                foreach (var name in ValueColumns(schema, enrichment))
                    Copy(row, name, Get(entry.Value, name));
            rows.Add(row);
        }
        return [.. rows.OrderBy(row => Get(row, mapKey)?.ToString() ?? "", StringComparer.Ordinal)];
    }

    static bool TryStamp(object? current, string by, object value)
    {
        if (current is null or string or Value or IEnumerable)
            return false;
        return TrySet(current, by, value) || TrySet(current, "Id", value) || TrySet(current, "id", value);
    }

    static void Copy(object? dest, string name, object? value)
    {
        if (dest is IDictionary dict)
        {
            dict[name] = value;
            return;
        }
        if (dest is null || value is null)
            return;
        TrySet(dest, name, value);
    }

    static bool TrySet(object obj, string name, object? value)
    {
        var prop = obj.GetType().GetProperty(name);
        if (prop is null) return false;
        Write(prop, obj, value);
        return true;
    }

    static void Write(PropertyInfo prop, object obj, object? value)
    {
        var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        prop.SetValue(obj, value is null || target.IsInstanceOfType(value)
            ? value
            : Convert.ChangeType(value, target));
    }

    static object? NewMapValue(IDictionary map)
    {
        var args = map.GetType().GetGenericArguments();
        return args.Length == 2 ? Activator.CreateInstance(args[1]) : null;
    }

    static PropertyInfo Prop<TItem>(TItem obj, string name) =>
        obj!.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");

    internal static string MapKey(Enrichment enrichment) =>
        enrichment.DictionaryKey ?? throw new DatasourceException($"{enrichment.DestinationProperty} requires \"DictionaryKey\"");

    static IEnumerable<string> ValueColumns(Schema schema, Enrichment enrichment) =>
        schema.Columns
            .Where(pair => !pair.Value.IsId && pair.Key != enrichment.RelatedKey && pair.Key != MapKey(enrichment))
            .Select(pair => pair.Key);
}
