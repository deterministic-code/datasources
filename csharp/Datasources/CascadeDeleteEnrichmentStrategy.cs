namespace Datasources;

public sealed class CascadeDeleteEnrichmentStrategy<T> : DeleteEnrichmentStrategy<T>
{
    public override async Task DeleteChildren<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        T parent,
        Func<IReadOnlyList<TChild>, Task> deleteNested)
    {
        var existing = await Load(writer, enrichment, parent);
        await deleteNested(existing);
        await DeleteExisting(writer, enrichment, parent, existing);
    }

    /// <summary>Deletes already-loaded children of <paramref name="parent"/>, without cascading into their children.</summary>
    internal static async Task DeleteExisting<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent, List<TChild> existing)
        where TChild : new()
    {
        foreach (var row in existing)
            await writer.Delete(row, ChildrenWhere(writer, enrichment, parent));
    }

    public override Task DeleteLinked<TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        T parent) =>
        DeleteLinkedWith(writer, enrichment, parent, null);

    /// <summary><see cref="DeleteLinked{TChild}"/> for a parent whose current link rows were already loaded (null loads them).</summary>
    internal static async Task DeleteLinkedWith<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent, List<TChild>? loaded)
        where TChild : new()
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        if (Get(parent, enrichment.SourceProperty) is not { } raw) return;
        var parentKey = Value.From(raw);
        foreach (var row in loaded ?? await LoadLinks(writer, association.Parent, parentKey))
            await writer.Delete(row, LinkWhere(association, parentKey, Value.From(Get(row, association.Related))));
    }
}
