using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class TransactionLifecycleTests
{
    [Fact]
    public async Task Commit()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        await txn.Commit();
        Assert.NotNull(txn);
    }

    [Fact]
    public async Task Abort()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        await txn.Abort();
        Assert.NotNull(txn);
    }
}
