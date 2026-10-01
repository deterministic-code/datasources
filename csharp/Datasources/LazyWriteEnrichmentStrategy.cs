namespace Datasources;

public sealed class LazyWriteEnrichmentStrategy<T> : DefaultWriteEnrichmentStrategy<T>
{
    public LazyWriteEnrichmentStrategy(
        DeleteEnrichmentStrategy<T>? delete = null,
        UpdateEnrichmentStrategy<T>? update = null,
        CreateEnrichmentStrategy<T>? create = null,
        bool useTransaction = false) : base(delete, update, create, useTransaction) { }

    protected override bool SkipUnloaded => true;
}
