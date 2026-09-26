using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class BulkCRUDRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
        public string label { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    sealed class Child
    {
        public int id { get; set; }
        public int parent_id { get; set; }
        public string label { get; set; } = "";
    }

    [Fact]
    public async Task CreateWritesParentsAndChildren()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        var response = await Repo(parentsRepo, childrenRepo).Create(
        [
            new Parent { children = [new Child { label = "a" }] },
            new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
        ]);
        var parents = await parentsRepo.Read().ToListAsync();
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, parents.Count);
        Assert.All(parents, parent => Assert.True(parent.id > 0));
        Assert.Equal(3, children.Count);
        Assert.Equal(parents.Select(parent => parent.id), children.Select(child => child.parent_id).Distinct());
        Assert.Equal(2, response.RowsAffected);
    }

    [Fact]
    public async Task BulkCreateMatchesListCreate()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        await parentsRepo.BulkCreate([new Parent { label = "a" }, new Parent { label = "b" }]);
        Assert.Equal(2, (await parentsRepo.Read().ToListAsync()).Count);
    }

    [Fact]
    public async Task UpdateReplaceSyncsEachParent()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        await parentsRepo.Create(new Parent { label = "one" });
        await parentsRepo.Create(new Parent { label = "two" });
        var parents = await parentsRepo.Read().ToListAsync();
        var first = parents.Single(parent => parent.label == "one");
        var second = parents.Single(parent => parent.label == "two");
        await childrenRepo.Create(new Child { parent_id = first.id, label = "a" });
        await childrenRepo.Create(new Child { parent_id = second.id, label = "b" });
        var kept = (await childrenRepo.Read().ToListAsync()).Single(child => child.parent_id == first.id);
        await Repo(parentsRepo, childrenRepo).BulkUpdate(
        [
            new Parent { id = first.id, label = "one", children = [new Child { id = kept.id, label = "aa" }] },
            new Parent { id = second.id, label = "two", children = [new Child { label = "c" }] }
        ]);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.Equal("aa", children.Single(child => child.parent_id == first.id).label);
        Assert.Equal("c", children.Single(child => child.parent_id == second.id).label);
        Assert.DoesNotContain(children, child => child.label == "b");
    }

    [Fact]
    public async Task DeleteCascadesEachParent()
    {
        var connection = await Setup();
        var parentsRepo = new BulkCRUDRepository<Parent>(connection);
        var childrenRepo = new BulkCRUDRepository<Child>(connection);
        await parentsRepo.Create(new Parent { label = "one" });
        await parentsRepo.Create(new Parent { label = "two" });
        var parents = await parentsRepo.Read().ToListAsync();
        var first = parents.Single(parent => parent.label == "one");
        var second = parents.Single(parent => parent.label == "two");
        await childrenRepo.Create(new Child { parent_id = first.id, label = "a" });
        await childrenRepo.Create(new Child { parent_id = second.id, label = "b" });
        await childrenRepo.Create(new Child { label = "keep" });
        await Repo(parentsRepo, childrenRepo).BulkDelete([new Parent { id = first.id }, new Parent { id = second.id }]);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Equal("keep", Assert.Single(await childrenRepo.Read().ToListAsync()).label);
    }

    static BulkCRUDRepository<Parent> Repo(
        BulkCRUDRepository<Parent> parentsRepo,
        BulkCRUDRepository<Child> childrenRepo)
    {
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        return parentsRepo;
    }

    static async Task<IConnection> Setup()
    {
        var connection = new MemoryProvider().Connection(null);
        await connection.Open();
        return connection;
    }
}
