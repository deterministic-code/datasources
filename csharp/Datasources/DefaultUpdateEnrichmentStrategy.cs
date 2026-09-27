namespace Datasources;

public sealed class DefaultUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
{
    public override async Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
    {
        var writer = entry.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        var existing = await DeleteEnrichmentStrategy<T>.Load(writer, enrichment, parent);
        var payload = Payload(parent, enrichment.Into);
        foreach (var related in payload)
            Stamp(related, enrichment.By, Get(parent, enrichment.From));
        var keys = writer.Ids;
        var keep = payload.Where(row => HasKey(row, keys)).Select(row => KeyOf(row, keys)).ToHashSet();
        foreach (var row in existing.Where(row => !keep.Contains(KeyOf(row, keys))))
            await writer.Delete(row);
        foreach (var related in payload)
        {
            if (HasKey(related, keys) && existing.Any(row => KeyOf(row, keys) == KeyOf(related, keys)))
                await writer.Update(related);
            else
                await writer.Create(related);
        }
    }

    public override async Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var writer = entry.Link
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        var parentKey = Required(parent, enrichment.From, enrichment.Into);
        var existing = await DeleteEnrichmentStrategy<T>.LoadLinks(writer, association.Parent, parentKey);
        var wanted = RelatedIds(parent, enrichment);
        var keep = wanted.Select(id => id.ToString()).ToHashSet();
        foreach (var row in existing)
        {
            var id = Get(row, association.Related);
            if (id?.ToString() is { } key && keep.Contains(key)) continue;
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.LinkWhere(association, parentKey, id));
        }
        foreach (var id in wanted)
        {
            if (existing.Any(row => Get(row, association.Related)?.ToString() == id.ToString()))
                continue;
            var row = writer.New();
            Set(row, association.Parent, parentKey);
            Set(row, association.Related, id);
            await writer.Create(row);
        }
    }
}
