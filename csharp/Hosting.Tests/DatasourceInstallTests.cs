namespace Datasources.Tests;

public class DatasourceInstallTests
{
    static readonly string[] EnvKeys =
        ["INSTALLED_PROVIDERS", "providers", "DATABASE_URL", "url", "DATABASE_SCHEMA", "provider"];

    public DatasourceInstallTests()
    {
        Datasource.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
    }

    [Fact]
    public void ProviderNamesFromListAndDefault()
    {
        Assert.Equal(
            new[] { "sqlite", "json", "postgres" },
            ProviderNames.From(new Dictionary<string, string?>
            {
                ["providers"] = "sqlite, json, sqlite",
                ["provider"] = "postgres"
            }));
        Assert.Equal(
            new[] { "json", "memory" },
            ProviderNames.From(new Dictionary<string, string?>
            {
                ["INSTALLED_PROVIDERS"] = "json,memory",
                ["url"] = "json://"
            }));
        Assert.Equal(
            new[] { "sqlite" },
            ProviderNames.From(new Dictionary<string, string?> { ["provider"] = " sqlite " }));
        Assert.Empty(ProviderNames.From(new Dictionary<string, string?>()));
    }

    [Fact]
    public void InstallsListedFactoriesAndDefaults()
    {
        DatasourceInstall.From(new Dictionary<string, string?>
        {
            ["providers"] = "json,memory",
            ["provider"] = "memory",
            ["filename"] = ":memory:"
        });
        Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
        Assert.Equal("memory", Datasource.Create().Provider);
        Assert.Equal("json", Datasource.Create("json").Provider);
    }

    [Fact]
    public void DefaultsRegistersInstalledProviders()
    {
        Datasource.Defaults(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["provider"] = "memory"
        });
        Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
        Assert.Equal("memory", Datasource.Create().Provider);
    }

    [Fact]
    public void CreateRegistersInstalledProviders()
    {
        Assert.Equal("json", Datasource.Create(null, new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["provider"] = "json"
        }).Provider);
        Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
    }

    [Fact]
    public void CreateReadsInstalledProvidersFromEnvironment()
    {
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "json,memory");
        Environment.SetEnvironmentVariable("provider", "memory");
        try
        {
            Assert.Equal("memory", Datasource.Create().Provider);
            Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
        }
        finally
        {
            Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", null);
            Environment.SetEnvironmentVariable("provider", null);
        }
    }

    [Fact]
    public void MissingPackageOrUnknownName()
    {
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["provider"] = "sqlite" }));
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["provider"] = "nope" }));
    }
}
