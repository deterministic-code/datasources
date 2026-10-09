namespace Datasources;

public sealed class DefaultCreateEnrichmentStrategy<T> : CreateEnrichmentStrategy<T>
{
    public override async Task CreateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
    {
        foreach (var related in Children(writer, enrichment, parent))
        {
            Stamp(related, enrichment.RelatedKey, Get(parent, enrichment.SourceProperty));
            await writer.Create(related);
        }
    }

    public override async Task CreateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKey = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty);
        foreach (var id in RelatedIds(parent, enrichment))
        {
            var row = new TChild();
            Set(row, association.Parent, parentKey.ToObject());
            Set(row, association.Related, id.ToObject());
            await writer.Create(row);
        }
    }
}
