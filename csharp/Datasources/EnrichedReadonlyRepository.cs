namespace Datasources;

public class EnrichedReadonlyRepository<T> : ReadonlyRepository<T>, IEnriched<T>
{
    readonly IReadonlyRepository<T> inner;
    readonly IReadonlyEnrichmentStrategy<T> strategy;

    public EnrichedReadonlyRepository(
        IReadonlyRepository<T> inner,
        IReadonlyEnrichmentStrategy<T>? strategy = null)
    {
        this.inner = inner;
        this.strategy = strategy ?? new DefaultReadonlyEnrichmentStrategy<T>();
        Tree = new RepositoryTree(new RepositoryTreeNode(inner, "", binder: this.strategy));
    }

    public IRepositoryTree Tree { get; }

    public IEnriched<T> AddEnriched<TRelated>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        string? select = null,
        bool many = false,
        Action<IRepositoryTreeNode>? nest = null)
    {
        
        var child = Tree.Parent.Include(via, into, from, by, select, many);
        nest?.Invoke(child);
        return this;
    }

    public Task Enrich(IList<T> items) => strategy.Enrich(Tree, items);

    public override IAsyncEnumerable<T> Read(IStatement? query = null) =>
        strategy.Read(inner, Tree, query);

    public override async IAsyncEnumerable<T> Read(IWhere where)
    {
        var items = new List<T>();
        await foreach (var item in inner.Read(where))
            items.Add(item);
        await Enrich(items);
        foreach (var item in items)
            yield return item;
    }

    public override async Task<T?> ReadSingle(IWhere where)
    {
        await foreach (var item in Read(where))
            return item;
        return default;
    }
}
