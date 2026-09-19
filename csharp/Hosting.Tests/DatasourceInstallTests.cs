namespace Datasources.Tests;

public class DatasourceInstallTests
{
    static readonly string[] EnvKeys = ["INSTALLED_PROVIDERS", "DATABASE_URL", "DATABASE_SCHEMA"];

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
    public void InstallsListedFactories()
    {
        DatasourceInstall.From(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["DATABASE_URL"] = "memory://",
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
            ["DATABASE_URL"] = "memory://"
        });
        Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
        Assert.Equal(
            "memory",
            DatasourceFactory.Create(new Dictionary<string, string?> { ["database_url"] = "memory://" }).Provider);
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
        Environment.SetEnvironmentVariable("DATABASE_URL", "memory://");
        try
        {
            Assert.Equal("memory", DatasourceFactory.Create().Provider);
            Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
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
