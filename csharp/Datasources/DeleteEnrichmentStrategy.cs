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

    /// <summary>Deletes the children of every parent. Loops <see cref="DeleteChildren{TChild}"/>; batching strategies override it.</summary>
    public virtual async Task DeleteChildrenBatch<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        IReadOnlyList<T> parents,
        Func<IReadOnlyList<TChild>, Task> deleteNested)
        where TChild : new()
    {
        foreach (var parent in parents)
            await DeleteChildren(writer, enrichment, parent, deleteNested);
    }

    /// <summary>Deletes the links of every parent. Loops <see cref="DeleteLinked{TChild}"/>; batching strategies override it.</summary>
    public virtual async Task DeleteLinkedBatch<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        IReadOnlyList<T> parents)
        where TChild : new()
    {
        foreach (var parent in parents)
            await DeleteLinked(writer, enrichment, parent);
    }

    /// <summary>
    /// Loads, with one <c>IN</c> read per chunk of keys, the rows of <paramref name="writer"/> whose
    /// <paramref name="column"/> equals each parent's <paramref name="source"/> cell. The result lines up with
    /// <paramref name="parents"/>; a parent with no key or no rows gets an empty list.
    /// </summary>
    internal static async Task<List<List<TChild>>> LoadGrouped<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        string column,
        string source,
        IReadOnlyList<TParent> parents)
    {
        var keys = parents.Select(parent => Get(parent, source)).ToList();
        var distinct = new List<Value>();
        var seen = new HashSet<string>();
        foreach (var key in keys)
            if (key is not null && seen.Add(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(key)))
                distinct.Add(Value.From(key));
        var groups = new Dictionary<string, List<TChild>>();
        foreach (var chunk in distinct.Chunk(500))
            await foreach (var item in writer.Read(new Where(new Operator.In(column, chunk))))
            {
                if (Get(item, column) is not { } owner) continue;
                var cacheKey = DefaultReadonlyEnrichmentStrategy<object>.KeyOf(owner);
                if (!groups.TryGetValue(cacheKey, out var list))
                    groups[cacheKey] = list = [];
                list.Add(item);
            }
        return keys
            .Select(key => key is not null && groups.TryGetValue(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(key), out var list)
                ? list
                : new List<TChild>())
            .ToList();
    }

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
