namespace Datasources;

public abstract class UpdateEnrichmentStrategy<T>
{
    public abstract Task UpdateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    public abstract Task UpdateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    protected static List<Value> RelatedIds<TParent>(TParent parent, Enrichment enrichment) =>
        EnrichmentCells.RelatedIds(parent, enrichment);

    protected static Value Required<TParent>(TParent parent, string name, string into) =>
        EnrichmentCells.Required(parent, name, into);

    protected static bool Missing(Value? value) => EnrichmentCells.Missing(value);

    protected static string Text(Value value) => EnrichmentCells.Text(value);

    protected static IReadOnlyList<TItem> Payload<TParent, TItem>(TParent parent, string into) =>
        EnrichmentCells.Payload<TParent, TItem>(parent, into);

    protected static IReadOnlyList<TChild> Children<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
        where TChild : new() =>
        EnrichmentCells.Children(writer, enrichment, parent);

    protected static void Stamp<TItem>(TItem related, string by, object? key) =>
        EnrichmentCells.Stamp(related, by, key);

    protected static bool HasKey<TItem>(TItem item, IReadOnlyList<string> keys) =>
        keys.Count > 0 && keys.All(name => Get(item, name) is not null and not 0 and not 0L and not 0u and not "");

    protected static string KeyOf<TItem>(TItem item, IReadOnlyList<string> keys) =>
        string.Join("\u001f", keys.Select(name => Get(item, name)?.ToString() ?? ""));

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    protected static void Set<TItem>(TItem obj, string name, object? value) =>
        EnrichmentCells.Set(obj, name, value);
}
