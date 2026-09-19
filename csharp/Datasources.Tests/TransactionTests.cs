using Datasources;

namespace Datasources.Tests;

public class TransactionTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "DATABASE_SCHEMA",
        "DEFAULT_PROVIDER",
        "DEFAULT_DATASOURCE_URL",
        "PROVIDER",
    ];

    public TransactionTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("sqlite", _ => new MockConnection(), c => new MockRepository<object>(c));
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
    }

    [Fact]
    public async Task StartFromDefault()
    {
        var ds = DatasourceFactory.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var txn = await connection.StartTransaction();
        Assert.Equal(IsolationLevel.Serializable, txn.Isolation);
    }
}
