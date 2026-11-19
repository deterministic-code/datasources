namespace Datasources;

public abstract class DeleteEnrichmentStrategy<T>
{
    public abstract Task DeleteChildren<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        T parent,
        Func<IReadOnlyList<TChild>, Task> deleteNested)
        where TChild : new();

    public abstract Task DeleteLinked<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        T parent)
        where TChild : new();

    internal static async Task<List<TChild>> Load<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
    {
        var clause = MatchWhere(enrichment, parent);
        if (clause is null) return [];
        var items = new List<TChild>();
        await foreach (var item in writer.Read(clause))
            items.Add(item);
        return items;
    }

    internal static async Task<List<TChild>> LoadLinks<TChild>(
        ICRUDRepository<TChild> writer,
        string column,
        Value key)
    {
        var items = new List<TChild>();
        await foreach (var item in writer.Read(new Where(new Operator.Equals(column, key))))
            items.Add(item);
        return items;
    }

    internal static IWhere? ChildrenWhere<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent) =>
        writer.Ids.Count > 0 ? null : MatchWhere(enrichment, parent);

    internal static IWhere LinkWhere(Association association, Value parentKey, Value relatedKey) =>
        new Where(new Operator.And(
            new Operator.Equals(association.Parent, parentKey),
            new Operator.Equals(association.Related, relatedKey)));

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    static Where? MatchWhere<TParent>(Enrichment enrichment, TParent parent)
    {
        Condition? condition = null;
        foreach (var pair in enrichment.Pairs())
        {
            var key = Get(parent, pair.SourceProperty);
            if (key is null) return null;
            Condition equals = new Operator.Equals(pair.RelatedKey, Value.From(key));
            condition = condition is null ? equals : new Operator.And(condition, equals);
        }
        return condition is null ? null : new Where(condition);
    }
}
