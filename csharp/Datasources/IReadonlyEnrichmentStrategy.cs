namespace Datasources;

public interface IReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentBinder where T : new();
