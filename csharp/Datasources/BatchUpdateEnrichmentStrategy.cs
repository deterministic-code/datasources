namespace Datasources;

/// <summary>
/// Same writes as <see cref="DefaultUpdateEnrichmentStrategy{T}"/>, but the current children or links of all
/// parents are loaded with one batched read instead of one read per parent. Enrichments keyed by several
/// columns load per parent.
/// </summary>
public sealed class BatchUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
{
    readonly DefaultUpdateEnrichmentStrategy<T> single = new();

    public override Task UpdateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) =>
        single.UpdateChildren(writer, enrichment, parent);

    public override Task UpdateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) =>
        single.UpdateLinked(writer, enrichment, parent);

    public override async Task UpdateChildrenBatch<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, IReadOnlyList<T> parents)
    {
        var pairs = enrichment.Pairs().ToList();
        if (parents.Count < 2 || pairs.Count != 1)
        {
            await base.UpdateChildrenBatch(writer, enrichment, parents);
            return;
        }
        var existing = await DeleteEnrichmentStrategy<T>.LoadGrouped(writer, pairs[0].RelatedKey, pairs[0].SourceProperty, parents);
        for (var i = 0; i < parents.Count; i++)
            await single.UpdateChildrenWith(writer, enrichment, parents[i], existing[i]);
    }

    public override async Task UpdateLinkedBatch<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, IReadOnlyList<T> parents)
    {
        if (parents.Count < 2 || enrichment.Association is not { } association)
        {
            await base.UpdateLinkedBatch(writer, enrichment, parents);
            return;
        }
        var existing = await DeleteEnrichmentStrategy<T>.LoadGrouped(writer, association.Parent, enrichment.SourceProperty, parents);
        for (var i = 0; i < parents.Count; i++)
            await single.UpdateLinkedWith(writer, enrichment, parents[i], existing[i]);
    }
}
