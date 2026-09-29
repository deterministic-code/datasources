using Datasources;

namespace Datasources.Tests;

[Collection("DatasourceFactory")]
public class TransactionTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public TransactionTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("sqlite", new MockDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
    }

    [Fact]
    public async Task CommitOrAbort()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        var saved = true;
        if (saved)
            await txn.Commit();
        else
            await txn.Abort();
        Assert.Equal(IsolationLevel.Serializable, txn.Isolation);
    }
}
