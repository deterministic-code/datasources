namespace Datasources;

public sealed class GraphCreateEnrichmentStrategy<T, TLink> : CreateEnrichmentStrategy<T>
    where TLink : new()
{
    readonly ICRUDRepository<TLink> link;
    readonly Dictionary<string, object> inserted = [];
    int depth;

    public GraphCreateEnrichmentStrategy(ICRUDRepository<TLink> link) => this.link = link;

    public override Task CreateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent) =>
        Task.CompletedTask;

    public override async Task CreateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        depth++;
        try
        {
            var parentId = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty).ToObject()!;
            inserted[NodeKey(parent)] = parentId;
            foreach (var child in Children(writer, enrichment, parent))
            {
                var key = NodeKey(child);
                object relatedId;
                if (inserted.TryGetValue(key, out var existing))
                    relatedId = existing;
                else
                {
                    var created = await writer.Create(child);
                    relatedId = Required(created, enrichment.SourceProperty, enrichment.DestinationProperty).ToObject()!;
                    inserted[key] = relatedId;
                    Stamp(child, enrichment.SourceProperty, relatedId);
                    if (child is T nested)
                        await CreateChildren(writer, enrichment, nested);
                }
                var row = new TLink();
                Set(row, association.Parent, parentId);
                Set(row, association.Related, relatedId);
                await link.Create(row);
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
        throw new DatasourceException("Graph create requires Name, Value, or Id");
    }
}
