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
    public void CreateDefaultFromEnvSettings()
    {
        var ds = Datasource.Default;
        var command = ds.CreateCommand();
        Assert.Equal("sqlite", ds.Provider);
        Assert.NotNull(command);
    }
}
