using Datasources;
using Datasources.Memory;

namespace Datasources.Memory.Tests;

[Collection("DatasourceFactory")]
public class TransactionLifecycleTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public TransactionLifecycleTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("memory", new MemoryDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "memory");
    }

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
    public async Task CommandRunningInOneAtomicTransaction()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
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
