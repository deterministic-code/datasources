namespace Datasources;

public sealed class StoredEnrichment
{
    internal StoredEnrichment(
        Enrichment enrichment,
        Func<IList<object>, IReadOnlyList<Row?>?, Task> apply,
        RepositoryTreeNode.IChildPersist? persist = null,
        RepositoryTreeNode.IChildPersist? link = null)
    {
        Enrichment = enrichment;
        Apply = apply;
        Persist = persist;
        Link = link;
    }

    public Enrichment Enrichment { get; }
    internal Func<IList<object>, IReadOnlyList<Row?>?, Task> Apply { get; }
    internal RepositoryTreeNode.IChildPersist? Persist { get; }
    internal RepositoryTreeNode.IChildPersist? Link { get; }
}
