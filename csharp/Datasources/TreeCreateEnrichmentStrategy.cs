namespace Datasources;

public sealed class TreeCreateEnrichmentStrategy<T> : CreateEnrichmentStrategy<T>
{
    readonly Dictionary<string, object> inserted = [];
    int depth;

    public override Task CreateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) =>
        Task.CompletedTask;

    public override async Task CreateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
    {
        depth++;
        try
        {
            inserted[NodeKey(parent)] = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty).ToObject()!;
            var children = Children(writer, enrichment, parent);
            foreach (var child in children)
            {
                foreach (var pair in enrichment.Pairs())
                    Stamp(child, pair.RelatedKey, Required(parent, pair.SourceProperty, enrichment.DestinationProperty).ToObject());
                var key = NodeKey(child);
                if (inserted.ContainsKey(key))
                    throw new DatasourceException($"Tree create cycle at \"{key}\"");
                var created = await writer.Create(child);
                var relatedId = Required(created, enrichment.SourceProperty, enrichment.DestinationProperty).ToObject()!;
                inserted[key] = relatedId;
                Stamp(child, enrichment.SourceProperty, relatedId);
                if (child is T nested)
                    await CreateChildren(writer, enrichment, nested);
            }
        }
        finally
        {
            depth--;
            if (depth == 0)
                inserted.Clear();
        }
    }

    static string NodeKey<TItem>(TItem item)
    {
        var name = Get(item, "Name") ?? Get(item, "Value");
        if (name is not null && !Missing(Value.From(name)))
            return Text(Value.From(name));
        var id = Get(item, "Id");
        if (id is not null && !Missing(Value.From(id)))
            return Text(Value.From(id));
        var region = Get(item, "RegionId");
        var category = Get(item, "CategoryId");
        if (region is not null && category is not null)
            return $"{Text(Value.From(region))}:{Text(Value.From(category))}";
        throw new DatasourceException("Tree create requires Name, Value, Id, or RegionId/CategoryId");
    }
}
