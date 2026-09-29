namespace Datasources.Tests;

public class DatasourceInstallTests
{
    static readonly string[] EnvKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA", "DEFAULT_PROVIDER", "DEFAULT_DATASOURCE_URL", "PROVIDER"];

    public DatasourceInstallTests()
    {
        DatasourceFactory.Reset();
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
                ["DATASOURCE_URL"] = "postgres://localhost"
            }));
        Assert.Equal(
            new[] { "json", "memory" },
            ProviderNames.From(new Dictionary<string, string?>
            {
                ["INSTALLED_PROVIDERS"] = "json,memory",
                ["DATASOURCE_URL"] = "json://"
            }));
        Assert.Equal(
            new[] { "sqlite" },
            ProviderNames.From(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = " sqlite " }));
        Assert.Equal(
            new[] { "postgresql" },
            ProviderNames.From(new Dictionary<string, string?> { ["DATASOURCE_URL"] = "postgresql://localhost/db" }));
        Assert.Empty(ProviderNames.From(new Dictionary<string, string?>()));
        var error = Assert.Throws<DatasourceException>(() =>
            ProviderNames.From(new Dictionary<string, string?> { ["DATASOURCE_URL"] = "not-a-url" }));
        Assert.Equal("DATASOURCE_URL has no provider", error.Message);
    }

    [Fact]
    public void InstallsListedFactories()
    {
        DatasourceInstall.From(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["DATASOURCE_URL"] = "memory://",
        });
        Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
        Assert.Equal(
            "memory",
            DatasourceFactory.Create(new Dictionary<string, string?>
            {
                ["provider"] = "memory",
                ["database_url"] = "memory://",
                ["filename"] = ":memory:"
            }).Provider);
        Assert.Equal(
            "json",
            DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "json" }).Provider);
    }

    [Fact]
    public void DefaultsRegistersInstalledProviders()
    {
        DatasourceFactory.Defaults(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["DATASOURCE_URL"] = "memory://"
        });
        Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
        Assert.Equal(
            "memory",
            DatasourceFactory.Create(new Dictionary<string, string?>
            {
                ["provider"] = "memory",
                ["database_url"] = "memory://"
            }).Provider);
    }

    [Fact]
    public void CreateDoesNotTakeInstalledProviders()
    {
        DatasourceInstall.From(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory"
        });
        Assert.Equal(
            "json",
            DatasourceFactory.Create(new Dictionary<string, string?>
            {
                ["provider"] = "json",
                ["database_url"] = "json://"
            }).Provider);
        Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
    }

    [Fact]
    public void CreateReadsConnectionFromEnvironment()
    {
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "json,memory");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "memory://");
        Environment.SetEnvironmentVariable("PROVIDER", "memory");
        try
        {
            Assert.Equal("memory", DatasourceFactory.Create().Provider);
            Assert.Equal("memory", DatasourceFactory.Default.Provider);
            Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
        }
        finally
        {
            Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", null);
            Environment.SetEnvironmentVariable("DATASOURCE_URL", null);
            Environment.SetEnvironmentVariable("PROVIDER", null);
        }
    }

    [Fact]
    public void DefaultOverridesEnv()
    {
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "json,memory");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "memory://");
        Environment.SetEnvironmentVariable("DEFAULT_PROVIDER", "json");
        Environment.SetEnvironmentVariable("DEFAULT_DATASOURCE_URL", "json://");
        try
        {
            Assert.Equal("json", DatasourceFactory.Default.Provider);
        }
        finally
        {
            Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", null);
            Environment.SetEnvironmentVariable("DATASOURCE_URL", null);
            Environment.SetEnvironmentVariable("DEFAULT_PROVIDER", null);
            Environment.SetEnvironmentVariable("DEFAULT_DATASOURCE_URL", null);
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
