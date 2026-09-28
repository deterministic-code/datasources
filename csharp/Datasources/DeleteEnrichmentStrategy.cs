using System.Reflection;

namespace Datasources;

public abstract class DeleteEnrichmentStrategy<T>
{
    public abstract Task DeleteOwned(
        StoredEnrichment entry,
        Enrichment enrichment,
        object parent,
        Func<IReadOnlyList<object>, Task> deleteNested);

    public abstract Task DeleteLinked(
        StoredEnrichment entry,
        Enrichment enrichment,
        object parent);

    internal static async Task<List<object>> Load(
        IChildPersist writer,
        Enrichment enrichment,
        object parent)
    {
        var key = Get(parent, enrichment.From);
        if (key is null) return [];
        var items = new List<object>();
        await foreach (var item in writer.Read(new Where(new Operator.Equals(enrichment.By, Value.From(key)))))
            items.Add(item);
        return items;
    }

    internal static async Task<List<object>> LoadLinks(
        IChildPersist writer,
        string column,
        object key)
    {
        var items = new List<object>();
        await foreach (var item in writer.Read(new Where(new Operator.Equals(column, Value.From(key)))))
            items.Add(item);
        return items;
    }

    internal static IWhere LinkWhere(Association association, object parentKey, object? relatedKey) =>
        new Where(new Operator.And(
            new Operator.Equals(association.Parent, Value.From(parentKey)),
            new Operator.Equals(association.Related, Value.From(relatedKey))));

    protected static object? Get(object obj, string name) => Prop(obj, name).GetValue(obj);

    static PropertyInfo Prop(object obj, string name) =>
        obj.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
