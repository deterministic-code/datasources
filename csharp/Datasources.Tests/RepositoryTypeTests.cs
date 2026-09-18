namespace Datasources.Tests;

public class RepositoryTypeTests
{
    sealed class Item
    {
        public int id { get; set; }
    }

    [Fact]
    public void RepositoryReportsClosedGenericType()
    {
        var repo = new Repository<Item>(new MockConnection { Provider = "memory" });
        Assert.Equal(typeof(Item), repo.Type);
        Assert.Equal(typeof(Item), ((IRepository)repo).Type);
    }

    [Fact]
    public void DeepCRUDRepositoryReportsSameTypeAsInner()
    {
        var inner = new ListCRUDRepository<Item>();
        var repo = new DeepCRUDRepository<Item>(inner);
        Assert.Equal(typeof(Item), inner.Type);
        Assert.Equal(typeof(Item), repo.Type);
    }

    [Fact]
    public void FactoryCreateByTypeExposesItemType()
    {
        var factory = new RepositoryFactory();
        factory.Register<Item>(_ => new StubRepository<Item>("memory", new StubConnection("memory")));
        var repo = (IRepository)factory.Create(typeof(Item));
        Assert.Equal(typeof(Item), repo.Type);
    }
}
