namespace Datasources;

public sealed class CascadeDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
{
    public override async Task DeleteOwned(
        IChildPersist writer,
        Enrichment enrichment,
        T parent,
        Func<IReadOnlyList<object>, Task> deleteNested)
    {
        var existing = await Load(writer, enrichment, parent);
        await deleteNested(existing);
        foreach (var row in existing)
            await writer.Delete(row);
    }

    public override async Task DeleteLinked(
        IChildPersist writer,
        Enrichment enrichment,
        T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKey = Get(parent, enrichment.SourceProperty);
        if (parentKey is null) return;
        foreach (var row in await LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, LinkWhere(association, parentKey, Get(row, association.Related)));
    }
}
