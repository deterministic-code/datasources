namespace Datasources;

public sealed class LazyWriteEnrichmentStrategy<T> : DefaultWriteEnrichmentStrategy<T>
{
    protected override bool SkipUnloaded => true;
}
