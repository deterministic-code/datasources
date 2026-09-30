using System.Collections;
using System.Reflection;

namespace Datasources;

public abstract class UpdateEnrichmentStrategy<T>
{
    public abstract Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent);

    public abstract Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent);

    internal async Task CreateChildren(IChildPersist writer, Enrichment enrichment, object parent)
    {
        foreach (var related in Payload(parent, enrichment.Into))
        {
            Stamp(related, enrichment.By, Get(parent, enrichment.From));
            await writer.Create(related);
        }
    }

    internal async Task CreateLinks(
        IChildPersist writer,
        Enrichment enrichment,
        Association association,
        object parent)
    {
        var parentKey = Required(parent, enrichment.From, enrichment.Into);
        foreach (var id in RelatedIds(parent, enrichment))
        {
            var row = writer.New();
            Set(row, association.Parent, parentKey);
            Set(row, association.Related, id);
            await writer.Create(row);
        }
    }

    protected static List<object> RelatedIds(object parent, Enrichment enrichment)
    {
        var ids = new List<object>();
        var seen = new HashSet<string>();
        foreach (var related in Payload(parent, enrichment.Into))
        {
            if (Get(related, enrichment.By) is not object id || Missing(id))
                throw new DatasourceException($"{enrichment.Into} requires \"{enrichment.By}\"");
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

    protected static void Stamp(object related, string by, object? key)
    {
        if (key is not null)
            Set(related, by, key);
    }

    protected static bool HasKey(object item, IReadOnlyList<string> keys) =>
        keys.Count > 0 && keys.All(name => Get(item, name) is not null and not 0 and not 0L and not 0u and not "");

    protected static string KeyOf(object item, IReadOnlyList<string> keys) =>
        string.Join("\u001f", keys.Select(name => Get(item, name)?.ToString() ?? ""));

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
