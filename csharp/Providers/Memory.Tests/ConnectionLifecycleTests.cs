using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class ConnectionLifecycleTests
{
    [Fact]
    public async Task OpenAndClose()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        await connection.Close();
        Assert.False(connection.IsOpen);
    }

    [Fact]
    public async Task StartTransaction()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        Assert.NotNull(txn);
    }
}
