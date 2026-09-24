namespace Datasources;

public class EnrichedReadonlyRepository<T> : ReadonlyRepository<T>, IEnriched<T> where T : new()
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

    public override IAsyncEnumerable<T> Read(IWhere where) =>
        Items(ReadTracked(where));

    protected internal override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        Sourced(Tracked(inner, query));

    protected internal override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IWhere where) =>
        Sourced(Tracked(inner, where));

    async IAsyncEnumerable<(T Item, Row? Source)> Sourced(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        var items = new List<T>();
        var sources = new List<Row?>();
        await foreach (var (item, source) in pairs)
        {
            items.Add(item);
            sources.Add(source);
        }
        await strategy.Enrich(Tree, items, sources);
        for (var i = 0; i < items.Count; i++)
            yield return (items[i], sources[i]);
    }

    static async IAsyncEnumerable<T> Items(IAsyncEnumerable<(T Item, Row? Source)> pairs)
    {
        await foreach (var (item, _) in pairs)
            yield return item;
    }

    public override async Task<T?> ReadSingle(IWhere where)
    {
        await foreach (var item in Read(where))
            return item;
        return default;
    }
}
