namespace Datasources.Tests;

public class DatasourceInstallTests
{
    public DatasourceInstallTests() => Datasource.Reset();

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
    public void MissingPackageOrUnknownName()
    {
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["provider"] = "sqlite" }));
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceInstall.From(new Dictionary<string, string?> { ["provider"] = "nope" }));
    }
}
