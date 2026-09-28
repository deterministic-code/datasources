namespace Datasources;

public sealed class UnitOfWorkTransaction
{
    static readonly HashSet<IConnection> open = new(ReferenceEqualityComparer.Instance);
    readonly IConnection connection;

    public UnitOfWorkTransaction(IConnection connection) => this.connection = connection;

    public async Task<TResult> DoWork<TResult>(Func<Task<TResult>> work)
    {
        var joined = false;
        lock (open)
            joined = !open.Add(connection);
        if (joined)
            return await work();

        try
        {
            var txn = await connection.StartTransaction();
            try
            {
                var result = await work();
                await txn.Commit();
                return result;
            }
            catch
            {
                await txn.Abort();
                throw;
            }
        }
        finally
        {
            lock (open)
                open.Remove(connection);
        }
    }
}
