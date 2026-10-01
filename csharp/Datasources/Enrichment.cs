namespace Datasources;

public sealed record Enrichment(
    string DestinationProperty,
    string SourceProperty,
    string RelatedKey,
    string? RelatedProperty = null,
    EnrichmentPropertyType Type = EnrichmentPropertyType.Object,
    Association? Association = null);
