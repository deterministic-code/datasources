namespace Datasources;

internal class StoredEnrichment
{
    internal StoredEnrichment(Enrichment enrichment) => Enrichment = enrichment;

    public Enrichment Enrichment { get; }
}

internal sealed class StoredEnrichment<T> : StoredEnrichment
{
    internal StoredEnrichment(
        Enrichment enrichment,
        ReadEnrichment<T> read,
        PersistEnrichment<T>? persist = null,
        LinkEnrichment<T>? link = null) : base(enrichment)
    {
        Read = read;
        Persist = persist;
        Link = link;
    }

    internal ReadEnrichment<T> Read { get; set; }

    internal PersistEnrichment<T>? Persist { get; set; }

    internal LinkEnrichment<T>? Link { get; set; }
}
