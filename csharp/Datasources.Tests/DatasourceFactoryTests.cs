using Datasources;

namespace Datasources.Tests;

[Collection("DatasourceFactory")]
public class DatasourceFactoryTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public DatasourceFactoryTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
    }

    [Fact]
    public void DefaultFromEnvironment()
    {
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "memory");
        Environment.SetEnvironmentVariable("PROVIDER", "memory");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "memory://");
        Assert.Equal("memory", DatasourceFactory.Default.Provider);
    }

    [Fact]
    public void MissingProviderThrows()
    {
        var error = Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "missing" }));
        Assert.Equal("missing", error.Provider);
    }
}
