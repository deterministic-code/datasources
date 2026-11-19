namespace Datasources;

public sealed record EnrichmentMatch(string SourceProperty, string RelatedKey);

public sealed record Enrichment(
    string DestinationProperty,
    string SourceProperty,
    string RelatedKey,
    string? RelatedProperty = null,
    EnrichmentPropertyType Type = EnrichmentPropertyType.Object,
    Association? Association = null,
    string? DictionaryKey = null,
    IReadOnlyList<EnrichmentMatch>? Keys = null)
{
    public IReadOnlyList<EnrichmentMatch> Pairs() =>
        Keys is { Count: > 0 } keys ? keys : [new EnrichmentMatch(SourceProperty, RelatedKey)];
}
