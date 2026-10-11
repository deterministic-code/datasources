namespace Datasources;

internal abstract class ReadEnrichment<T>
{
    internal abstract Task Apply(Enrichment enrichment, IList<T> parents, IReadOnlyList<Row?>? sources);
}

internal sealed class ReadEnrichment<T, TRelated> : ReadEnrichment<T>
{
    readonly IReadonlyRepository<TRelated> via;
    readonly IReadonlyEnrichmentStrategy<T> strategy;

    internal ReadEnrichment(IReadonlyRepository<TRelated> via, IReadonlyEnrichmentStrategy<T> strategy)
    {
        this.via = via;
        this.strategy = strategy;
    }

    internal override Task Apply(Enrichment enrichment, IList<T> parents, IReadOnlyList<Row?>? sources) =>
        strategy.Apply(via, enrichment, parents, sources);
}
