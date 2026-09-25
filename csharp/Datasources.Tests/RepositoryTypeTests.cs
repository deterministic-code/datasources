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
        var repo = new CRUDRepository<Item>(new MockConnection { Provider = "memory" });
        Assert.Equal(typeof(Item), repo.Type);
        Assert.Equal(typeof(Item), ((IRepository)repo).Type);
    }

    [Fact]
    public void CRUDRepositoryReportsSameTypeAsInner()
    {
        var repo = new ListCRUDRepository<Item>();
        Assert.Equal(typeof(Item), repo.Type);
    }

    [Fact]
    public void FactoryCreateByTypeExposesItemType()
    {
        var factory = new RepositoryFactory();
        factory.Register(new StubRepository<Item>("memory", new StubConnection("memory")));
        var repo = factory.Create<Item>();
        Assert.Equal(typeof(Item), repo.Type);
    }
}
