using Datasources;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class TreeGraphEnrichmentIntegrationTests
{
    public TreeGraphEnrichmentIntegrationTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class Category
    {
        public long Id { get; set; }
        public long? ParentCategoryId { get; set; }
        public string Name { get; set; } = "";
        public List<Category> Children { get; set; } = [];
    }

    [Fact]
    public async Task TreeReadStopsOnACycle()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native(
            "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"));
        await command.Execute(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (1, 3, 'food')"));
        await command.Execute(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (2, 1, 'fruit')"));
        await command.Execute(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (3, 2, 'apple')"));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var via = new ReadonlyRepository<Category>(connection, new ReadonlyRepositoryMappingOptions<Category>
        {
            Entity = "categories",
            PropertyMappings = mappings,
        });
        var repo = new ReadonlyRepository<Category>(connection, new ReadonlyRepositoryMappingOptions<Category>
        {
            Entity = "categories",
            PropertyMappings = mappings,
        });
        repo.AddEnriched(
            new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List),
            via,
            new TreeReadonlyEnrichmentStrategy<Category>());
        var food = await repo.ReadSingle(new Where("Id", eq, 1));
        Assert.Equal("fruit", food!.Children.Single().Name);
        Assert.Equal("apple", food.Children.Single().Children.Single().Name);
        Assert.Empty(food.Children.Single().Children.Single().Children);
    }
}
