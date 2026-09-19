namespace Datasources.Tests;

public class DatasourceInstallTests
{
    static readonly string[] EnvKeys = ["INSTALLED_PROVIDERS", "DATABASE_URL", "DATABASE_SCHEMA"];

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
                ["INSTALLED_PROVIDERS"] = "sqlite, json, sqlite",
                ["DATABASE_URL"] = "postgres://localhost"
            }));
        Assert.Equal(
            new[] { "json", "memory" },
            ProviderNames.From(new Dictionary<string, string?>
            {
                ["INSTALLED_PROVIDERS"] = "json,memory",
                ["DATABASE_URL"] = "json://"
            }));
        Assert.Equal(
            new[] { "sqlite" },
            ProviderNames.From(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = " sqlite " }));
        Assert.Empty(ProviderNames.From(new Dictionary<string, string?>()));
    }

    [Fact]
    public void InstallsListedFactoriesAndDefaults()
    {
        DatasourceInstall.From(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["DATABASE_URL"] = "memory://",
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
            ["DATABASE_URL"] = "memory://"
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
            ["DATABASE_URL"] = "json://"
        }).Provider);
        Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
    }

    [Fact]
    public void CreateReadsInstalledProvidersFromEnvironment()
    {
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "json,memory");
        Environment.SetEnvironmentVariable("DATABASE_URL", "memory://");
        try
        {
            Assert.Equal("memory", Datasource.Create().Provider);
            Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
        }
        finally
        {
            Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", null);
            Environment.SetEnvironmentVariable("DATABASE_URL", null);
        }
    }

    [Fact]
    public void MissingPackageOrUnknownName()
    {
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = "sqlite" }));
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = "nope" }));
    }
}
