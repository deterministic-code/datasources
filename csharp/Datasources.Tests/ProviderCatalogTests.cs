namespace Datasources.Tests;

[Collection("DatasourceFactory")]
public class ProviderCatalogTests
{
    static readonly string[] EnvKeys =
    [
        "INSTALLED_PROVIDERS",
        "DATASOURCE_URL",
        "PROVIDER",
    ];

    public ProviderCatalogTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
    }

    [Fact]
    public void DefaultStoresLocationAndModule()
    {
        var sqlite = ProviderCatalog.Default["sqlite"];
        Assert.Equal("Datasources.Sqlite", sqlite.Location);
        Assert.Equal("Datasources.Sqlite.SqliteDriver", sqlite.Module);
        Assert.Equal("Datasources.SqlServer.SqlServerDriver", ProviderCatalog.Default["sqlserver"].Module);
        Assert.Equal("Datasources.MongoDb.MongoDbDriver", ProviderCatalog.Default["mongodb"].Module);
        Assert.Equal("Datasources.DynamoDb.DynamoDbDriver", ProviderCatalog.Default["dynamodb"].Module);
    }

    [Fact]
    public void FullConstructorStoresLocationAndModule()
    {
        var catalog = new ProviderCatalog("acme", "Acme.Data", "Acme.Data.AcmeDriver");
        Assert.Equal("acme", catalog.Name);
        Assert.Equal("Acme.Data", catalog.Location);
        Assert.Equal("Acme.Data.AcmeDriver", catalog.Module);
    }

    [Fact]
    public void DefaultListsMonorepoProviders()
    {
        Assert.Equal(
            ["json", "memory", "sqlite", "xml", "yaml", "mysql", "sqlserver", "postgres", "oracle", "mongodb", "dynamodb"],
            ProviderCatalog.Default.Keys);
    }

    [Fact]
    public void DefaultsRegistersInstalledProviders()
    {
        DatasourceFactory.Defaults(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory",
            ["DATASOURCE_URL"] = "memory://"
        });
        Assert.Equal(["json", "memory"], DatasourceFactory.Registered());
        Assert.Equal(
            "memory",
            DatasourceFactory.Create(new Dictionary<string, string?>
            {
                ["PROVIDER"] = "memory",
                ["DATASOURCE_URL"] = "memory://"
            }).Provider);
    }

    [Fact]
    public void CreateDoesNotTakeInstalledProviders()
    {
        DatasourceFactory.Defaults(new Dictionary<string, string?>
        {
            ["INSTALLED_PROVIDERS"] = "json,memory"
        });
        Assert.Equal(
            "json",
            DatasourceFactory.Create(new Dictionary<string, string?>
            {
                ["PROVIDER"] = "json",
                ["DATASOURCE_URL"] = "json://"
            }).Provider);
        Assert.Equal(["json", "memory"], DatasourceFactory.Registered());
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
            Assert.Equal("memory", Datasource.Default.Provider);
            Assert.Equal(["json", "memory"], DatasourceFactory.Registered());
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
        Environment.SetEnvironmentVariable("PROVIDER", "json");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "json://");
        try
        {
            Assert.Equal("json", Datasource.Default.Provider);
        }
        finally
        {
            Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", null);
            Environment.SetEnvironmentVariable("DATASOURCE_URL", null);
            Environment.SetEnvironmentVariable("PROVIDER", null);
        }
    }

    [Fact]
    public void MissingPackageOrUnknownName()
    {
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceFactory.Defaults(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = "sqlite" }));
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceFactory.Defaults(new Dictionary<string, string?> { ["INSTALLED_PROVIDERS"] = "nope" }));
    }

    [Fact]
    public void UrlWithoutProviderThrows()
    {
        var error = Assert.Throws<DatasourceException>(() =>
            DatasourceFactory.Defaults(new Dictionary<string, string?> { ["DATASOURCE_URL"] = "not-a-url" }));
        Assert.Equal("DATASOURCE_URL has no provider", error.Message);
    }
}
