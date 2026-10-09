namespace Datasources;

public sealed class CascadeDeleteUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
{
    readonly CreateEnrichmentStrategy<T> create = new DefaultCreateEnrichmentStrategy<T>();

    public override async Task UpdateOwned(IChildPersist writer, Enrichment enrichment, T parent)
    {
        foreach (var row in await DeleteEnrichmentStrategy<T>.Load(writer, enrichment, parent))
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.OwnedWhere(writer, enrichment, parent));
        await create.CreateOwned(writer, enrichment, parent);
    }

    public override async Task UpdateLinked(IChildPersist writer, Enrichment enrichment, T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKey = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty);
        foreach (var row in await DeleteEnrichmentStrategy<T>.LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.LinkWhere(association, parentKey, Value.From(Get(row, association.Related))));
        await create.CreateLinked(writer, enrichment, parent);
    }
}
