namespace Datasources;

public class EnrichedReadonlyRepository<T> : ReadonlyRepository<T>, IEnriched<T>
{
    readonly IReadonlyRepository<T> inner;

    public EnrichedReadonlyRepository(IReadonlyRepository<T> inner)
    {
        this.inner = inner;
        Tree = new RepositoryTree(new RepositoryTreeNode(inner, ""));
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

    public Task Enrich(IList<T> items) =>
        Tree.Parent.EnrichFrom(items.Cast<object>().ToList());

    public override async IAsyncEnumerable<T> Get(IStatement? query = null)
    {
        var items = new List<T>();
        await foreach (var item in inner.Get(query))
            items.Add(item);
        await Enrich(items);
        foreach (var item in items)
            yield return item;
    }
}
