namespace Datasources;

internal interface IEnrichmentHost
{
    IReadOnlyList<StoredEnrichment> Enrichments { get; }
}
