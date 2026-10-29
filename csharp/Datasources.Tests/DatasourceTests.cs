using Datasources;

namespace Datasources.Tests;

[Collection("DatasourceFactory")]
public class DatasourceTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public DatasourceTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
        DatasourceFactory.Register("sqlite", new MockDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
    }

    [Fact]
    public async Task CreateDefaultFromEnvSettings()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        Assert.Equal("sqlite", ds.Provider);
        Assert.NotNull(command);
    }

    [Fact]
    public async Task ClosesTheDefaultConnection()
    {
        await using var ds = Datasource.Default;
        await ds.CreateCommand();
        Assert.True((await ds.CreateConnection()).IsOpen);
        await ds.Close();
        Assert.False((await ds.CreateConnection()).IsOpen);
    }
}
