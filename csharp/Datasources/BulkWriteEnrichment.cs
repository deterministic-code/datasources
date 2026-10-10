namespace Datasources;

static class BulkWriteEnrichment
{
    public static Task After<T>(
        CRUDRepository<T> repo,
        CRUDRepositoryOptions<T> options,
        IReadOnlyList<T> items,
        WriteMode mode)
        where T : new() =>
        items.Count == 0 || repo is not IEnrichmentHost host || host.Enrichments.Count == 0
            ? Task.CompletedTask
            : (options.Write ?? new DefaultWriteEnrichmentStrategy<T>()).PersistBulk(
                new RepositoryTree(new RepositoryTreeNode(repo, "")),
                items as IList<T> ?? [.. items],
                mode);
}
