namespace Datasources;

public interface IWriteEnrichmentStrategy<T>
{
    bool UseTransaction => false;

    Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode);

    Task PersistBulk(IRepositoryTree tree, IList<T> items, WriteMode mode) => Persist(tree, items, mode);

    Task ResolveLookups(IRepositoryTree tree, IList<T> items, Row record, Schema schema) => Task.CompletedTask;

    Task Lookup(IRepositoryTree tree, IList<T> items, Row record, Schema schema, Func<string, string> column) =>
        ResolveLookups(tree, items, record, schema);

    async Task<Row> LookupRow(
        IRepositoryTree tree,
        T item,
        Schema schema,
        Func<string, string> column,
        TypeConverter<T, Row> objects,
        TypeConverter<Row, Row>? rows)
    {
        var record = objects.ReverseConvert(item);
        await Lookup(tree, [item!], record, schema, column);
        return rows?.ReverseConvert(record) ?? record;
    }

    IReadOnlySet<string> Sources(IRepositoryTree tree, Func<string, string> column) => new HashSet<string>();

    bool Destination(IRepositoryTree tree, string name) => false;
}
