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

    sealed class RegionCategory
    {
        public long RegionId { get; set; }
        public long CategoryId { get; set; }
        public long? ParentRegionId { get; set; }
        public long? ParentCategoryId { get; set; }
        public string Name { get; set; } = "";
        public List<RegionCategory> Children { get; set; } = [];
    }

    sealed class Node
    {
        public long Id { get; set; }
        public string Value { get; set; } = "";
        public List<Node> Edges { get; set; } = [];
    }

    sealed class NodeEdge
    {
        public long Id { get; set; }
        public long FromNodeId { get; set; }
        public long ToNodeId { get; set; }
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

    [Fact]
    public async Task TreeReadReturnsEachRowOnceUnderTheRoots()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native(
            "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"));
        await command.Execute(new Native("INSERT INTO categories (id, parent_category_id, name) VALUES (1, NULL, 'food')"));
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
        var roots = await List(repo);
        Assert.Equal(["food"], roots.Select(row => row.Name));
        Assert.Equal(["food", "fruit", "apple"], Flatten(roots.Single()).ToList());
        var fruit = await repo.ReadSingle(new Where("Id", eq, 2));
        Assert.Equal("fruit", fruit!.Name);
        Assert.Equal("apple", fruit.Children.Single().Name);
    }

    [Fact]
    public async Task TreeReadReturnsTheCompositeRoot()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await connection.CreateCommand().Execute(new Native(
            """
            CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id)
            )
            """));
        await connection.CreateCommand().Execute(new Native(
            "INSERT INTO region_categories (region_id, category_id, name) VALUES (1, 10, 'food')"));
        await connection.CreateCommand().Execute(new Native(
            "INSERT INTO region_categories (region_id, category_id, parent_region_id, parent_category_id, name) VALUES (1, 11, 1, 10, 'fruit')"));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var via = new ReadonlyRepository<RegionCategory>(connection, new ReadonlyRepositoryMappingOptions<RegionCategory>
        {
            Entity = "region_categories",
            PropertyMappings = mappings,
        });
        var repo = new ReadonlyRepository<RegionCategory>(connection, new ReadonlyRepositoryMappingOptions<RegionCategory>
        {
            Entity = "region_categories",
            PropertyMappings = mappings,
        });
        repo.AddEnriched(
            new("Children", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [
                new EnrichmentMatch("RegionId", "ParentRegionId"),
                new EnrichmentMatch("CategoryId", "ParentCategoryId"),
            ]),
            via,
            new TreeReadonlyEnrichmentStrategy<RegionCategory>());
        var roots = await List(repo);
        Assert.Equal(["food"], roots.Select(row => row.Name));
        Assert.Equal("fruit", roots.Single().Children.Single().Name);
    }

    [Fact]
    public async Task TreeCreateThenReadMatches()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native(
            "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        CRUDRepositoryOptions<Category> Options() => new()
        {
            Entity = "categories",
            Ids = ["Id"],
            PropertyMappings = mappings,
        };
        var via = new CRUDRepository<Category>(connection, Options());
        var repo = new CRUDRepository<Category>(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<Category>(create: new TreeCreateEnrichmentStrategy<Category>()) });
        repo.AddEnriched(
            new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List),
            via,
            new TreeReadonlyEnrichmentStrategy<Category>());
        await repo.Create(new Category
        {
            Name = "food",
            Children = [new Category { Name = "fruit", Children = [new Category { Name = "apple" }] }],
        });
        var food = await repo.ReadSingle(new Where("Name", eq, "food"));
        Assert.Equal("fruit", food!.Children.Single().Name);
        Assert.Equal("apple", food.Children.Single().Children.Single().Name);
    }

    [Fact]
    public async Task TreeCreateThrowsOnACycle()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await connection.CreateCommand().Execute(new Native(
            "CREATE TABLE categories (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_category_id INTEGER, name TEXT NOT NULL)"));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        CRUDRepositoryOptions<Category> Options() => new()
        {
            Entity = "categories",
            Ids = ["Id"],
            PropertyMappings = mappings,
        };
        var via = new CRUDRepository<Category>(connection, Options());
        var repo = new CRUDRepository<Category>(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<Category>(create: new TreeCreateEnrichmentStrategy<Category>()) });
        repo.AddEnriched(
            new("Children", "Id", "ParentCategoryId", Type: EnrichmentPropertyType.List),
            via,
            new TreeReadonlyEnrichmentStrategy<Category>());
        var food = new Category { Name = "food" };
        var apple = new Category { Name = "apple", Children = [food] };
        food.Children = [new Category { Name = "fruit", Children = [apple] }];
        var error = await Assert.ThrowsAsync<DatasourceException>(() => repo.Create(food));
        Assert.Contains("food", error.Message);
    }

    [Fact]
    public async Task TreeCreateStampsCompositeParents()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        await connection.CreateCommand().Execute(new Native(
            """
            CREATE TABLE region_categories (
                region_id INTEGER NOT NULL,
                category_id INTEGER NOT NULL,
                parent_region_id INTEGER,
                parent_category_id INTEGER,
                name TEXT NOT NULL,
                PRIMARY KEY (region_id, category_id),
                FOREIGN KEY (parent_region_id, parent_category_id) REFERENCES region_categories (region_id, category_id)
            )
            """));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        CRUDRepositoryOptions<RegionCategory> Options() => new()
        {
            Entity = "region_categories",
            Ids = ["RegionId", "CategoryId"],
            PropertyMappings = mappings,
        };
        var via = new CRUDRepository<RegionCategory>(connection, Options());
        var repo = new CRUDRepository<RegionCategory>(connection, Options() with { Write = new DefaultWriteEnrichmentStrategy<RegionCategory>(create: new TreeCreateEnrichmentStrategy<RegionCategory>()) });
        repo.AddEnriched(
            new("Children", "RegionId", "ParentRegionId", Type: EnrichmentPropertyType.List, Keys: [
                new EnrichmentMatch("RegionId", "ParentRegionId"),
                new EnrichmentMatch("CategoryId", "ParentCategoryId"),
            ]),
            via,
            new TreeReadonlyEnrichmentStrategy<RegionCategory>());
        await repo.Create(new RegionCategory
        {
            RegionId = 1,
            CategoryId = 1,
            Name = "west",
            Children = [new RegionCategory { RegionId = 1, CategoryId = 2, Name = "coast" }],
        });
        var west = await repo.ReadSingle(new Where("Name", eq, "west"));
        Assert.Equal("coast", west!.Children.Single().Name);
        Assert.Equal(1, west.Children.Single().ParentRegionId);
        Assert.Equal(1, west.Children.Single().ParentCategoryId);
    }

    [Fact]
    public async Task GraphCreateWritesALoopThenRead()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native(
            "CREATE TABLE nodes (id INTEGER PRIMARY KEY AUTOINCREMENT, value TEXT NOT NULL)"));
        await command.Execute(new Native(
            "CREATE TABLE edges (id INTEGER PRIMARY KEY AUTOINCREMENT, from_node_id INTEGER NOT NULL, to_node_id INTEGER NOT NULL)"));
        var mappings = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        CRUDRepositoryOptions<Node> NodeOptions() => new()
        {
            Entity = "nodes",
            Ids = ["Id"],
            PropertyMappings = mappings,
        };
        CRUDRepositoryOptions<NodeEdge> EdgeOptions() => new()
        {
            Entity = "edges",
            Ids = ["Id"],
            PropertyMappings = mappings,
        };
        var via = new CRUDRepository<Node>(connection, NodeOptions());
        var link = new CRUDRepository<NodeEdge>(connection, EdgeOptions());
        var repo = new CRUDRepository<Node>(connection, NodeOptions() with { Write = new DefaultWriteEnrichmentStrategy<Node>(create: new GraphCreateEnrichmentStrategy<Node, NodeEdge>(link)) });
        repo.AddEnriched(
            new("Edges", "Id", "Id", Type: EnrichmentPropertyType.List, Association: new("FromNodeId", "ToNodeId")),
            via,
            new GraphReadonlyEnrichmentStrategy<Node, NodeEdge>(link));
        var a = new Node { Value = "a" };
        var b = new Node { Value = "b", Edges = [a] };
        a.Edges = [b];
        await repo.Create(a);
        var loadedA = await repo.ReadSingle(new Where("Value", eq, "a"));
        Assert.Equal("b", loadedA!.Edges.Single().Value);
        var loadedB = await repo.ReadSingle(new Where("Value", eq, "b"));
        Assert.Equal("a", loadedB!.Edges.Single().Value);
    }

    static async Task<List<T>> List<T>(IReadonlyRepository<T> repo)
    {
        var rows = new List<T>();
        await foreach (var row in repo.Read()) rows.Add(row);
        return rows;
    }

    static IEnumerable<string> Flatten(Category node)
    {
        yield return node.Name;
        foreach (var child in node.Children)
            foreach (var name in Flatten(child))
                yield return name;
    }
}
