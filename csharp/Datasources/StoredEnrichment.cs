namespace Datasources;

internal class StoredEnrichment
{
    internal StoredEnrichment(Enrichment enrichment, IChildPersist? persist = null, IChildPersist? link = null)
    {
        Enrichment = enrichment;
        Persist = persist;
        Link = link;
    }

    public Enrichment Enrichment { get; }
    internal IChildPersist? Persist { get; private set; }
    internal IChildPersist? Link { get; private set; }

    internal void AttachPersist(IChildPersist persist) => Persist = persist;

    internal void AttachLink(IChildPersist link) => Link = link;
}

internal sealed class StoredEnrichment<T> : StoredEnrichment
{
    internal StoredEnrichment(
        Enrichment enrichment,
        Func<IList<T>, IReadOnlyList<Row?>?, Task> apply,
        IChildPersist? persist = null,
        IChildPersist? link = null) : base(enrichment, persist, link) =>
        Apply = apply;

    internal Func<IList<T>, IReadOnlyList<Row?>?, Task> Apply { get; private set; }

    internal void AttachLink(Func<IList<T>, IReadOnlyList<Row?>?, Task> apply, IChildPersist link)
    {
        Apply = apply;
        base.AttachLink(link);
    }
}
