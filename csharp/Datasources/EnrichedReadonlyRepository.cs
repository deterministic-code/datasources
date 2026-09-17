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

    public override IAsyncEnumerable<T> Get(IStatement? query = null) =>
        strategy.Get(inner, Tree, query);
}
