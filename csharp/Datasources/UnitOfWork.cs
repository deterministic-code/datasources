namespace Datasources;

public sealed class UnitOfWork
{
    readonly IConnection connection;

    public UnitOfWork(IConnection connection) => this.connection = connection;

    public async Task<TResult> InTransaction<TResult>(Func<Task<TResult>> work)
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
}
