using Datasources;

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
        var repo = new ListReadonlyRepository<Item>();
        readonlyFactory.Register(repo);

        Assert.Same(repo, readonlyFactory.Create<Item>());
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
        Assert.Contains("Item", missing.Message);
    }

    [Fact]
    public void MissingThrows()
    {
        var factory = new RepositoryFactory();
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
    }
}
