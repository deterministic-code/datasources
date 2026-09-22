using Datasources;

namespace Datasources.Tests;

public class DatasourceFactoryTests
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

    public DatasourceFactoryTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
    }

    [Fact]
    public void DefaultFromEnvironment()
    {
        DatasourceFactory.Register("sqlite", new MockDriver());
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        var ds = DatasourceFactory.Default;
        Assert.Equal("sqlite", ds.Provider);
    }

    [Fact]
    public void MissingProviderThrows()
    {
        var error = Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "missing" }));
        Assert.Equal("missing", error.Provider);
    }
}
