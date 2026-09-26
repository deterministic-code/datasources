namespace Datasources;

public sealed class StoredEnrichment
{
    internal StoredEnrichment(
        Enrichment enrichment,
        Func<IList<object>, IReadOnlyList<Row?>?, Task> apply,
        IChildPersist? persist = null,
        IChildPersist? link = null)
    {
        Enrichment = enrichment;
        Apply = apply;
        Persist = persist;
        Link = link;
    }

    public Enrichment Enrichment { get; }
    internal Func<IList<object>, IReadOnlyList<Row?>?, Task> Apply { get; private set; }
    internal IChildPersist? Persist { get; private set; }
    internal IChildPersist? Link { get; private set; }

    internal void AttachPersist(IChildPersist persist) => Persist = persist;

    internal void AttachLink(
        Func<IList<object>, IReadOnlyList<Row?>?, Task> apply,
        IChildPersist link)
    {
        Apply = apply;
        Link = link;
    }
}
