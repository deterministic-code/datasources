namespace Datasources;

public sealed class DefaultUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
{
    public override async Task UpdateOwned(IChildPersist writer, Enrichment enrichment, T parent)
    {
        var existing = await DeleteEnrichmentStrategy<T>.Load(writer, enrichment, parent);
        var payload = Owned(writer, enrichment, parent);
        foreach (var related in payload)
            Stamp(related, enrichment.RelatedKey, Get(parent, enrichment.SourceProperty));
        var keys = writer.Ids;
        var keep = payload.Where(row => HasKey(row, keys)).Select(row => KeyOf(row, keys)).ToHashSet();
        foreach (var row in existing.Where(row => !keep.Contains(KeyOf(row, keys))))
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.OwnedWhere(writer, enrichment, parent));
        foreach (var related in payload)
        {
            if (HasKey(related, keys) && existing.Any(row => KeyOf(row, keys) == KeyOf(related, keys)))
                await writer.Update(related);
            else
                await writer.Create(related);
        }
    }

    public override async Task UpdateLinked(IChildPersist writer, Enrichment enrichment, T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKey = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty);
        var existing = await DeleteEnrichmentStrategy<T>.LoadLinks(writer, association.Parent, parentKey);
        var wanted = RelatedIds(parent, enrichment);
        var keep = wanted.Select(Text).ToHashSet();
        foreach (var row in existing)
        {
            var id = Value.From(Get(row, association.Related));
            if (id is not Value.Null && keep.Contains(Text(id))) continue;
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.LinkWhere(association, parentKey, id));
        }
        foreach (var id in wanted)
        {
            if (existing.Any(row => Text(Value.From(Get(row, association.Related))) == Text(id)))
                continue;
            var row = writer.New();
            Set(row, association.Parent, parentKey.ToObject());
            Set(row, association.Related, id.ToObject());
            await writer.Create(row);
        }
    }
}
