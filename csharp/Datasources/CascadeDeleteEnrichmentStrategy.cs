namespace Datasources;

public sealed class CascadeDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
{
    public override async Task DeleteOwned(
        StoredEnrichment entry,
        Enrichment enrichment,
        object parent,
        Func<IReadOnlyList<object>, Task> deleteNested)
    {
        var writer = entry.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        var existing = await Load(writer, enrichment, parent);
        await deleteNested(existing);
        foreach (var row in existing)
            await writer.Delete(row);
    }

    public override async Task DeleteLinked(
        StoredEnrichment entry,
        Enrichment enrichment,
        object parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var writer = entry.Link
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        var parentKey = Get(parent, enrichment.From);
        if (parentKey is null) return;
        foreach (var row in await LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, LinkWhere(association, parentKey, Get(row, association.Related)));
    }
}
