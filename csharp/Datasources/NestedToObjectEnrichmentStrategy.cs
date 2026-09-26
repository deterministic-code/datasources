namespace Datasources;

public sealed class NestedToObjectEnrichmentStrategy<T>(TypeMap typeMap) : DefaultReadonlyEnrichmentStrategy<T>
    where T : new()
{
    readonly FlatToNestedMapper nested = new(typeMap);
    readonly MappedValueMapper<T> values = new();

    public override Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null)
    {
        parents.Clear();
        foreach (var item in nested.Map(Rows(sources)))
            parents.Add(values.Map(item)!);
        return Task.CompletedTask;
    }

    static IEnumerable<Row> Rows(IReadOnlyList<Row?>? sources) =>
        sources?.OfType<Row>() ?? [];
}
