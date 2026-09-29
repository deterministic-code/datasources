using Datasources;
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

    [Fact]
    public async Task TwoCommandsRunningTogether()
    {
        var connection = new MemoryConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        var command = connection.CreateCommand();
        var fields = new Parameter[] { new("name", "ada") };
        await command.Execute(new InsertStatement("people", fields), fields);
        var reader = await command.ExecuteReader(new SelectInsertedIdStatement("people", "id"));
        var moved = await reader.Next();
        var id = reader.Current!["id"];
        await txn.Commit();
        Assert.True(moved);
        Assert.Equal(new Value.Int(1), id);
    }
}
