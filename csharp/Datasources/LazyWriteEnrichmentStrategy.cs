namespace Datasources;

public sealed class LazyWriteEnrichmentStrategy<T> : DefaultWriteEnrichmentStrategy<T>
{
    public LazyWriteEnrichmentStrategy(
        DeleteEnrichmentStrategy<T>? delete = null,
        UpdateEnrichmentStrategy<T>? update = null,
        bool useTransaction = false) : base(delete, update, useTransaction) { }

    protected override bool SkipUnloaded => true;
}
