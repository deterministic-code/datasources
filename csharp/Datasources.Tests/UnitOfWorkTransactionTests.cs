using Datasources;

namespace Datasources.Tests;

public class UnitOfWorkTransactionTests
{
    [Fact]
    public async Task NestedCallStartsOnce()
    {
        var connection = new MockConnection();
        var txn = (MockTransaction)connection.Transaction;
        await new UnitOfWorkTransaction(connection).DoWork(async () =>
        {
            await new UnitOfWorkTransaction(connection).DoWork(() => Task.FromResult(0));
            return 0;
        });
        Assert.Equal(1, connection.StartTransactionCalls);
        Assert.Equal(1, txn.CommitCalls);
        Assert.Equal(0, txn.AbortCalls);
    }
}
