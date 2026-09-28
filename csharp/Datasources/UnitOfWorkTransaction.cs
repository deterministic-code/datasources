namespace Datasources;

public sealed class UnitOfWorkTransaction : IUnitOfWork
{
    static readonly HashSet<IConnection> open = new(ReferenceEqualityComparer.Instance);
    readonly IConnection connection;
    ITransaction? transaction;
    bool begun;

    public UnitOfWorkTransaction(IConnection connection) => this.connection = connection;

    public async Task Begin()
    {
        if (begun)
            throw new DatasourceException("Unit of work already begun");
        begun = true;
        var owner = false;
        lock (open)
            owner = open.Add(connection);
        if (!owner)
            return;
        try
        {
            transaction = await connection.StartTransaction();
        }
        catch
        {
            lock (open)
                open.Remove(connection);
            throw;
        }
    }

    public async Task Commit()
    {
        if (!begun)
            throw new DatasourceException("Unit of work has not begun");
        if (transaction is not { } txn)
            return;
        await txn.Commit();
        Release();
    }

    public async Task Abort()
    {
        if (!begun)
            throw new DatasourceException("Unit of work has not begun");
        if (transaction is not { } txn)
            return;
        try
        {
            await txn.Abort();
        }
        finally
        {
            Release();
        }
    }

    void Release()
    {
        lock (open)
            open.Remove(connection);
        transaction = null;
    }
}
