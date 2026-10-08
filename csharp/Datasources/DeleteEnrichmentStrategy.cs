using System.Reflection;

namespace Datasources;

public abstract class DeleteEnrichmentStrategy<T>
{
    public abstract Task DeleteOwned(
        IChildPersist writer,
        Enrichment enrichment,
        T parent,
        Func<IReadOnlyList<object>, Task> deleteNested);

    public abstract Task DeleteLinked(
        IChildPersist writer,
        Enrichment enrichment,
        T parent);

    internal static async Task<List<object>> Load<TParent>(
        IChildPersist writer,
        Enrichment enrichment,
        TParent parent)
    {
        var key = Get(parent, enrichment.SourceProperty);
        if (key is null) return [];
        var items = new List<object>();
        await foreach (var item in writer.Read(new Where(new Operator.Equals(enrichment.RelatedKey, Value.From(key)))))
            items.Add(item);
        return items;
    }

    internal static async Task<List<object>> LoadLinks(
        IChildPersist writer,
        string column,
        Value key)
    {
        var items = new List<object>();
        await foreach (var item in writer.Read(new Where(new Operator.Equals(column, key))))
            items.Add(item);
        return items;
    }

    internal static IWhere LinkWhere(Association association, Value parentKey, Value relatedKey) =>
        new Where(new Operator.And(
            new Operator.Equals(association.Parent, parentKey),
            new Operator.Equals(association.Related, relatedKey)));

    protected static object? Get<TItem>(TItem obj, string name) => Prop(obj, name).GetValue(obj);

    static PropertyInfo Prop<TItem>(TItem obj, string name) =>
        obj!.GetType().GetProperty(name)
        ?? throw new DatasourceException($"Unknown property \"{name}\"");
}
