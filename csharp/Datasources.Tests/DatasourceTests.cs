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
    public async Task CreatesFromPluralName()
    {
        var ds = new Datasource("mock", new MockDriver());
        var connection = new StubConnection("mock");
        var crud = new MockRepository<Item>(connection);
        var read = new MockRepository<Item>(connection);
        ds.Repositories.Register("Contact", "Contacts", _ => crud, _ => read);
        dynamic dsDynamic = ds;

        Item created = await dsDynamic.Contacts.Create(new Item());
        IStatement? query = null;
        Item? loaded = await dsDynamic.Contacts.ReadSingle(query);
        Response updated = await dsDynamic.Contacts.Update(created, null);
        Response deleted = await dsDynamic.Contacts.Delete(created, null);
        Assert.IsType<Item>(created);
        Assert.IsType<Item>(loaded);
        Assert.Equal(1, updated.RowsAffected);
        Assert.Equal(1, deleted.RowsAffected);
        Assert.Same(crud, ds.CreateRepository<Item>("Contact"));
        Assert.Same(read, ds.CreateReadonlyRepository<Item>("Contacts"));
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

    sealed class Item { }
}
