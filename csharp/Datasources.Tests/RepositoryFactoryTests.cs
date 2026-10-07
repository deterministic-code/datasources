using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class RepositoryFactoryTests
{
    sealed class Item { }

    [Fact]
    public void RegistersAndCreates()
    {
        var factory = new RepositoryFactory();
        var repo = new StubRepository<Item>("memory", new StubConnection("memory"));
        factory.Register(repo);
        var created = factory.Create<Item>();

        Assert.Same(repo, created);
        Assert.Same(repo, ((IRepositoryFactory)factory).Create<Item>());
        Assert.Same(repo, ((ICRUDRepositoryFactory)factory).Create<Item>());
    }

    [Fact]
    public void ReadonlyRegistrationIsNotCrud()
    {
        var factory = new RepositoryFactory();
        IRepositoryFactory readonlyFactory = factory;
        var repo = new ReadonlyRepository<Item>(new MemoryDriver().Connection(null));
        readonlyFactory.Register(repo);

        Assert.Same(repo, readonlyFactory.Create<Item>());
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
        Assert.Contains("Item", missing.Message);
    }

    [Fact]
    public void RegistersAndCreatesByName()
    {
        var factory = new RepositoryFactory();
        var connection = new StubConnection("memory");
        var crud = new StubRepository<Item>("crud", connection);
        var read = new StubRepository<Item>("read", connection);
        factory.Register("Contact", "Contacts", _ => crud, _ => read);

        Assert.Same(crud, factory.Create("Contact", connection));
        Assert.Same(read, factory.CreateReadonly("Contacts", connection));
        Assert.Same(crud, factory.Access("Contacts", connection).Create());
        Assert.Same(read, factory.Access("Contact", connection).CreateReadonly());
    }

    [Fact]
    public void MissingThrows()
    {
        var factory = new RepositoryFactory();
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
    }
}
