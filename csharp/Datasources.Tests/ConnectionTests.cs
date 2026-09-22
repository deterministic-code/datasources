using Datasources;

namespace Datasources.Tests;

public class ConnectionTests
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

    public ConnectionTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("sqlite", new MockDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
    }

    [Fact]
    public async Task CreateFromDefault()
    {
        var ds = DatasourceFactory.Default;
        var connection = await ds.CreateConnection();
        Assert.False(connection.IsOpen);
    }
}
