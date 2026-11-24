namespace Datasources;

/// <summary>
/// Same deletes as <see cref="CascadeDeleteEnrichmentStrategy{T}"/>, but the children or links of all parents
/// are loaded with one batched read, and the nested cascade runs once over all their rows. Enrichments keyed by
/// several columns delete per parent.
/// </summary>
public sealed class BatchDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
{
    readonly CascadeDeleteEnrichmentStrategy<T> single = new();

    public override Task DeleteChildren<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        T parent,
        Func<IReadOnlyList<TChild>, Task> deleteNested) =>
        single.DeleteChildren(writer, enrichment, parent, deleteNested);

    public override Task DeleteLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) =>
        single.DeleteLinked(writer, enrichment, parent);

    public override async Task DeleteChildrenBatch<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        IReadOnlyList<T> parents,
        Func<IReadOnlyList<TChild>, Task> deleteNested)
    {
        var pairs = enrichment.Pairs().ToList();
        if (parents.Count < 2 || pairs.Count != 1)
        {
            await base.DeleteChildrenBatch(writer, enrichment, parents, deleteNested);
            return;
        }
        var existing = await LoadGrouped(writer, pairs[0].RelatedKey, pairs[0].SourceProperty, parents);
        await deleteNested(existing.SelectMany(rows => rows).ToList());
        for (var i = 0; i < parents.Count; i++)
            await CascadeDeleteEnrichmentStrategy<T>.DeleteExisting(writer, enrichment, parents[i], existing[i]);
    }

    public override async Task DeleteLinkedBatch<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, IReadOnlyList<T> parents)
    {
        if (parents.Count < 2 || enrichment.Association is not { } association)
        {
            await base.DeleteLinkedBatch(writer, enrichment, parents);
            return;
        }
        var existing = await LoadGrouped(writer, association.Parent, enrichment.SourceProperty, parents);
        for (var i = 0; i < parents.Count; i++)
            await CascadeDeleteEnrichmentStrategy<T>.DeleteLinkedWith(writer, enrichment, parents[i], existing[i]);
    }
}
