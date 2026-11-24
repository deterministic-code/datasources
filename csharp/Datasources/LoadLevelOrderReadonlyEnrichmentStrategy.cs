namespace Datasources;

public class LoadLevelOrderReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
{
    const int DefaultBatchSize = 500;
    readonly int batchSize;

    public LoadLevelOrderReadonlyEnrichmentStrategy() : this(DefaultBatchSize) { }

    public LoadLevelOrderReadonlyEnrichmentStrategy(int batchSize) =>
        this.batchSize = batchSize < 1
            ? throw new DatasourceException("batchSize must be at least 1")
            : batchSize;

    public LoadLevelOrderReadonlyEnrichmentStrategy<T> WithBatchSize(int batchSize) => new(batchSize);

    protected override int BatchSize => batchSize;
}
