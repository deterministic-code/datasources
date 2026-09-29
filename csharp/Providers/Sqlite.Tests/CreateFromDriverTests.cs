using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class CreateFromDriverTests
{
    [Fact]
    public void CreateFromDriver()
    {
        var driver = new SqliteDriver();
        var ds = new Datasource("sqlite", driver, new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var repo = ds.CreateRepository<Item>();
        Assert.IsType<CRUDRepository<Item>>(repo);
    }

    sealed class Item
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
    }
}
