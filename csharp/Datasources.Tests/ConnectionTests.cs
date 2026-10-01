using Datasources;

namespace Datasources.Tests;

[Collection("DatasourceFactory")]
public class ConnectionTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public ConnectionTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("sqlite", new MockDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
    }

    [Fact]
    public async Task CreateFromDefault()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await connection.Close();
        Assert.False(connection.IsOpen);
    }
}
