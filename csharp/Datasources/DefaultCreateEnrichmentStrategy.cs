namespace Datasources;

public sealed class DefaultCreateEnrichmentStrategy<T> : CreateEnrichmentStrategy<T>
{
    public override async Task CreateOwned(IChildPersist writer, Enrichment enrichment, object parent)
    {
        foreach (var related in Payload(parent, enrichment.DestinationProperty))
        {
            Stamp(related, enrichment.RelatedKey, Get(parent, enrichment.SourceProperty));
            await writer.Create(related);
        }
    }

    public override async Task CreateLinked(IChildPersist writer, Enrichment enrichment, object parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKey = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty);
        foreach (var id in RelatedIds(parent, enrichment))
        {
            var row = writer.New();
            Set(row, association.Parent, parentKey);
            Set(row, association.Related, id);
            await writer.Create(row);
        }
    }
}
