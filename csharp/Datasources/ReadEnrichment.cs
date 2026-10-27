namespace Datasources;

internal abstract class ReadEnrichment<T>
{
    internal abstract Task Apply(Enrichment enrichment, IList<T> parents, IReadOnlyList<Row?>? sources);

    internal virtual Task<Value?> Lookup(string property, Value name, string key) => Task.FromResult<Value?>(null);
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

    internal override async Task<Value?> Lookup(string property, Value name, string key)
    {
        var found = await via.ReadSingle(new Where(new Operator.Equals(property, name)));
        return found is null ? null : Value.From(typeof(TRelated).GetProperty(key)?.GetValue(found));
    }
}
