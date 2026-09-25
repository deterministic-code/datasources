namespace Datasources;

public sealed class NestedToObjectEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
{
    readonly IMapper<IEnumerable<Row>, IReadOnlyList<T>> mapper;

    public NestedToObjectEnrichmentStrategy(IMapper<IEnumerable<Row>, IReadOnlyList<T>> mapper) =>
        this.mapper = mapper;

    public override Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null)
    {
        var rows = new List<Row>();
        if (sources is not null)
            foreach (var source in sources)
                if (source is { } row)
                    rows.Add(row);
        parents.Clear();
        foreach (var item in mapper.Map(rows))
            parents.Add(item!);
        return Task.CompletedTask;
    }
}
