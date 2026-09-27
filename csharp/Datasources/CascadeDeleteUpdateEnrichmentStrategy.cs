namespace Datasources;

public sealed class CascadeDeleteUpdateEnrichmentStrategy<T> : UpdateEnrichmentStrategy<T>
{
    public override async Task UpdateOwned(StoredEnrichment entry, Enrichment enrichment, object parent)
    {
        var writer = entry.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        foreach (var row in await DeleteEnrichmentStrategy<T>.Load(writer, enrichment, parent))
            await writer.Delete(row);
        await CreateChildren(writer, enrichment, parent);
    }

    public override async Task UpdateLinked(StoredEnrichment entry, Enrichment enrichment, object parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var writer = entry.Link
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
        var parentKey = Required(parent, enrichment.From, enrichment.Into);
        foreach (var row in await DeleteEnrichmentStrategy<T>.LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, DeleteEnrichmentStrategy<T>.LinkWhere(association, parentKey, Get(row, association.Related)));
        await CreateLinks(writer, enrichment, association, parent);
    }
}
