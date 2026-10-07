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
    public void CreatesFromPluralName()
    {
        var ds = new Datasource("mock", new MockDriver());
        var connection = new StubConnection("mock");
        var crud = new StubRepository<object>("crud", connection);
        var read = new StubRepository<object>("read", connection);
        ds.Repositories.Register("Contact", "Contacts", _ => crud, _ => read);
        dynamic dsDynamic = ds;

        Assert.Same(crud, dsDynamic.Contacts.Create());
        Assert.Same(read, dsDynamic.Contacts.CreateReadonly());
        Assert.Same(crud, ds.CreateRepository("Contact"));
        Assert.Same(read, ds.CreateReadonlyRepository("Contacts"));
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => _ = dsDynamic.Missing);
        Assert.Equal("Missing", missing.EntityTypeName);
    }

    [Fact]
    public async Task CreateDefaultFromEnvSettings()
    {
        var ds = Datasource.Default;
        var command = await ds.CreateCommand();
        Assert.Equal("sqlite", ds.Provider);
        Assert.NotNull(command);
    }
}
