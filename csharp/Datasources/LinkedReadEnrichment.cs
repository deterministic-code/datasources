namespace Datasources;

internal sealed class LinkedReadEnrichment<T, TRelated, TLink> : ReadEnrichment<T>
{
    readonly IReadonlyRepository<TRelated> via;
    readonly IReadonlyRepository<TLink> link;
    readonly IReadonlyEnrichmentStrategy<T> strategy;

    internal LinkedReadEnrichment(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TLink> link,
        IReadonlyEnrichmentStrategy<T> strategy)
    {
        this.via = via;
        this.link = link;
        this.strategy = strategy;
    }

    internal override Task Apply(Enrichment enrichment, IList<T> parents, IReadOnlyList<Row?>? sources) =>
        strategy.ApplyLinked(via, link, enrichment, parents, sources);
}
