using Datasources;

namespace Datasources.Tests;

public class UnitOfWorkTransactionTests
{
    [Fact]
    public async Task NestedCallStartsOnce()
    {
        var connection = new MockConnection();
        var txn = (MockTransaction)connection.Transaction;
        var outer = new UnitOfWorkTransaction(connection);
        await outer.Begin();
        var inner = new UnitOfWorkTransaction(connection);
        await inner.Begin();
        await inner.Commit();
        await outer.Commit();
        Assert.Equal(1, connection.StartTransactionCalls);
        Assert.Equal(1, txn.CommitCalls);
        Assert.Equal(0, txn.AbortCalls);
    }

    [Fact]
    public async Task NestedAbortRollsBackOnce()
    {
        var connection = new MockConnection();
        var txn = (MockTransaction)connection.Transaction;
        var outer = new UnitOfWorkTransaction(connection);
        await outer.Begin();
        var inner = new UnitOfWorkTransaction(connection);
        await inner.Begin();
        await inner.Abort();
        await outer.Abort();
        Assert.Equal(1, connection.StartTransactionCalls);
        Assert.Equal(0, txn.CommitCalls);
        Assert.Equal(1, txn.AbortCalls);
    }
}
