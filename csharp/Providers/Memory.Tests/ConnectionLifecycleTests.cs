using Datasources;
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
    public async Task OpenTwiceThrows()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var error = await Assert.ThrowsAsync<DatasourceException>(() => connection.Open());
        Assert.Equal("Connection is already open", error.Message);
    }

    [Fact]
    public async Task StartTransaction()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        Assert.NotNull(txn);
    }

    [Fact]
    public async Task CreateAndRunCommand()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        var response = await command.Execute("insert");
        Assert.Equal(1, response.RowsAffected);
    }

    [Fact]
    public async Task CloseWithUsing()
    {
        await using var connection = new MemoryConnection();
        await connection.Open();
    }
}
