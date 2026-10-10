namespace Datasources;

public class BulkCRUDRepository<T> : CRUDRepository<T> where T : new()
{
    readonly IBulkWriteStrategy<T> bulk;
    readonly bool useTransaction;

    public BulkCRUDRepository(IConnection connection) :
        this(connection, CreateCRUDRepositoryOptions.FromType<T>()) { }

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options) :
        this(connection, options, new SingleCommandIBulkWriteStrategy<T>()) { }

    public BulkCRUDRepository(IConnection connection, CRUDRepositoryOptions<T> options, IBulkWriteStrategy<T> bulk)
        : base(connection, options)
    {
        this.bulk = bulk;
        useTransaction = options.Write?.UseTransaction ?? false;
        bulk.Repo = this;
        bulk.Attach(Command(), options);
    }

    public async Task<IReadOnlyList<T>> Create(IReadOnlyList<T> items)
    {
        await InTransaction(() => bulk.Create(items), required: items.Count > 0);
        return items;
    }

    public Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        InTransaction(() => bulk.Update(items, byWhere));

    public Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere = null) =>
        InTransaction(() => bulk.Delete(items, byWhere));

    async Task<TResult> InTransaction<TResult>(Func<Task<TResult>> write, bool required = false)
    {
        if (!required && !useTransaction) return await write();
        var work = new UnitOfWorkTransaction(Command().Connection);
        await work.Begin();
        try
        {
            var result = await write();
            await work.Commit();
            return result;
        }
        catch
        {
            await work.Abort();
            throw;
        }
    }
}
