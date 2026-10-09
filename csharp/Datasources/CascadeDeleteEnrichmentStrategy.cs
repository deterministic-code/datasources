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
            await writer.Delete(row, OwnedWhere(writer, enrichment, parent));
    }

    public override async Task DeleteLinked(
        IChildPersist writer,
        Enrichment enrichment,
        T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        if (Get(parent, enrichment.SourceProperty) is not { } raw) return;
        var parentKey = Value.From(raw);
        foreach (var row in await LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, LinkWhere(association, parentKey, Value.From(Get(row, association.Related))));
    }
}
