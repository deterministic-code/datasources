using Datasources;

namespace Datasources.Tests;

public class RepositoryBuilderOptionsFactoryTests
{
    sealed class Item
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }

    [Fact]
    public void ForReturnsTheDialectOrTheDefault()
    {
        var fallback = new CRUDRepositoryOptions<Item> { CreateProperties = ["Name"] };
        var mysql = new CRUDRepositoryOptions<Item> { CreateProperties = ["Id", "Name"] };
        var factory = new RepositoryBuilderOptionsFactory<Item>().Add(fallback).Add("mysql", mysql);
        Assert.Same(mysql, factory.For("mysql"));
        Assert.Same(fallback, factory.For("sqlite"));
    }

    [Fact]
    public void BuilderSelectsByConnectionProvider()
    {
        var fallback = new CRUDRepositoryOptions<Item> { CreateProperties = ["Name"] };
        var mysql = new CRUDRepositoryOptions<Item> { CreateProperties = ["Id"] };
        var builder = new CRUDRepositoryBuilder<Item>().Add(fallback).Add("mysql", mysql);
        Assert.Same(mysql, builder.For(new MockConnection { Provider = "mysql" }));
        Assert.Same(fallback, builder.For(new MockConnection { Provider = "sqlite" }));
    }

    [Fact]
    public void ForThrowsWhenNoOptionsWereAdded() =>
        Assert.Equal(
            "repository options required",
            Assert.Throws<DatasourceException>(() => new RepositoryBuilderOptionsFactory<Item>().For("mysql")).Message);
}
