using Datasources;

namespace Datasources.Tests;

public class RepositoryFactoryTests
{
    sealed class Item { }

    [Fact]
    public void RegistersAndCreatesByTypeAndName()
    {
        var factory = new RepositoryFactory();
        var repo = new StubRepository<Item>("memory", new StubConnection("memory"));
        factory.Register<Item>(_ => repo);

        Assert.Same(repo, factory.Create<Item>());
        Assert.Same(repo, factory.Create("Item"));
        Assert.Same(repo, factory.Create(typeof(Item)));
        Assert.Same(repo, ((IRepositoryFactory)factory).Create<Item>());
        Assert.Same(repo, ((ICRUDRepositoryFactory)factory).Create<Item>());
        Assert.Same(repo, ((ICRUDRepositoryFactory)factory).Create("Item"));
        Assert.Same(repo, ((ICRUDRepositoryFactory)factory).Create(typeof(Item)));
    }

    [Fact]
    public void RegistersByNameAndType()
    {
        var factory = new RepositoryFactory();
        var named = new StubRepository<Item>("memory", new StubConnection("memory"));
        var typed = new StubRepository<Item>("json", new StubConnection("json"));
        factory.Register("people", _ => named);
        factory.Register(typeof(Item), _ => typed);

        Assert.Same(named, factory.Create("people"));
        Assert.Same(typed, factory.Create(typeof(Item)));
        Assert.Same(typed, factory.Create<Item>());
    }

    [Fact]
    public void ForwardsConnection()
    {
        var factory = new RepositoryFactory();
        IConnection? seen = null;
        var connection = new StubConnection("memory");
        factory.Register<Item>(conn =>
        {
            seen = conn;
            return new StubRepository<Item>("memory", conn!);
        });

        factory.Create<Item>(connection);
        Assert.Same(connection, seen);
        factory.Create("Item", connection);
        Assert.Same(connection, seen);
    }

    [Fact]
    public void ReadonlyRegistrationIsNotCrud()
    {
        var factory = new RepositoryFactory();
        IRepositoryFactory readonlyFactory = factory;
        var repo = new ListReadonlyRepository<Item>();
        readonlyFactory.Register<Item>(_ => repo);

        Assert.Same(repo, readonlyFactory.Create<Item>());
        Assert.Same(repo, factory.Create("Item"));
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
        Assert.Contains("Item", missing.Message);
        Assert.Throws<RepositoryNotRegisteredException>(() =>
            ((ICRUDRepositoryFactory)factory).Create("Item"));
    }

    [Fact]
    public void MissingThrows()
    {
        var factory = new RepositoryFactory();
        var missing = Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create<Item>());
        Assert.Equal("Item", missing.EntityTypeName);
        Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create("missing"));
        Assert.Throws<RepositoryNotRegisteredException>(() => factory.Create(typeof(Item)));
    }
}
