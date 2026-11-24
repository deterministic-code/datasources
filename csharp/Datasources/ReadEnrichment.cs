namespace Datasources;

internal abstract class ReadEnrichment<T>
{
    internal abstract Task Apply(Enrichment enrichment, IList<T> parents, IReadOnlyList<Row?>? sources);

    internal bool Selected(Enrichment enrichment, EnrichmentSelection? enrich) =>
        enrich is not { } selection || selection.Includes(enrichment.DestinationProperty);

    internal Task ApplySelected(
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources,
        EnrichmentSelection? enrich)
    {
        if (!Selected(enrichment, enrich)) return Task.CompletedTask;
        return Apply(enrichment, parents, sources);
    }

    internal virtual Task<Value?> Lookup(string property, Value name, string key) => Task.FromResult<Value?>(null);

    /// <summary>Resolves every name in one batched read; names with no match are absent from the result.</summary>
    internal virtual async Task<Dictionary<string, Value>> LookupBatch(string property, IReadOnlyList<Value> names, string key)
    {
        var found = new Dictionary<string, Value>();
        foreach (var name in names)
            if (await Lookup(property, name, key) is { } value)
                found[DefaultReadonlyEnrichmentStrategy<object>.KeyOf(name)] = value;
        return found;
    }

    /// <summary>
    /// One <c>IN</c> read per chunk of names; the first row per name wins, as in a single-row lookup.
    /// A name the batch read does not return falls back to the single-name read, so results match.
    /// </summary>
    internal static async Task<Dictionary<string, Value>> LookupBatchVia<TRelated>(
        IReadonlyRepository<TRelated> via,
        string property,
        IReadOnlyList<Value> names,
        string key)
    {
        var found = new Dictionary<string, Value>();
        foreach (var chunk in names.Chunk(500))
            await foreach (var item in via.Read(new Where(new Operator.In(property, chunk))))
            {
                if (EnrichmentCells.Get(item, property) is not { } name || EnrichmentCells.Get(item, key) is not { } value)
                    continue;
                found.TryAdd(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(name), Value.From(value));
            }
        foreach (var name in names)
        {
            if (found.ContainsKey(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(name))) continue;
            var row = await via.ReadSingle(new Where(new Operator.Equals(property, name)));
            if (row is not null && EnrichmentCells.Get(row, key) is { } value)
                found[DefaultReadonlyEnrichmentStrategy<object>.KeyOf(name)] = Value.From(value);
        }
        return found;
    }
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
        return found is null ? null : Value.From(EnrichmentCells.Get(found, key));
    }

    internal override Task<Dictionary<string, Value>> LookupBatch(string property, IReadOnlyList<Value> names, string key) =>
        LookupBatchVia(via, property, names, key);
}
