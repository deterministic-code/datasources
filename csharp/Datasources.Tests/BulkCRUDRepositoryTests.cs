using Datasources;

namespace Datasources.Tests;

public class BulkCRUDRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
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
        var parents = new ListCRUDRepository<Parent>();
        var children = new ListCRUDRepository<Child>();
        var repo = Repo(parents, children);
        var response = await repo.Create(
        [
            new Parent { children = [new Child { label = "a" }] },
            new Parent { children = [new Child { label = "b" }, new Child { label = "c" }] }
        ]);
        Assert.Equal(2, parents.Items.Count);
        Assert.Equal([1, 2], parents.Items.Select(parent => parent.id));
        Assert.Equal(3, children.Items.Count);
        Assert.Equal([1, 2, 2], children.Items.Select(child => child.parent_id));
        Assert.Equal(2, response.RowsAffected);
    }

    [Fact]
    public async Task BulkCreateMatchesListCreate()
    {
        var parents = new ListCRUDRepository<Parent>();
        var repo = new BulkCRUDRepository<Parent>(parents);
        await repo.BulkCreate([new Parent(), new Parent()]);
        Assert.Equal(2, parents.Items.Count);
    }

    [Fact]
    public async Task UpdateReplaceSyncsEachParent()
    {
        var parents = new ListCRUDRepository<Parent>(
            new Parent { id = 1 },
            new Parent { id = 2 });
        var children = new ListCRUDRepository<Child>(
            new Child { id = 1, parent_id = 1, label = "a" },
            new Child { id = 2, parent_id = 2, label = "b" });
        var repo = Repo(parents, children);
        await repo.BulkUpdate(
        [
            new Parent { id = 1, children = [new Child { id = 1, label = "aa" }] },
            new Parent { id = 2, children = [new Child { label = "c" }] }
        ]);
        Assert.Equal(2, children.Items.Count);
        Assert.Equal("aa", children.Items.Single(child => child.parent_id == 1).label);
        Assert.Equal("c", children.Items.Single(child => child.parent_id == 2).label);
        Assert.DoesNotContain(children.Items, child => child.id == 2);
    }

    [Fact]
    public async Task DeleteCascadesEachParent()
    {
        var parents = new ListCRUDRepository<Parent>(
            new Parent { id = 1 },
            new Parent { id = 2 });
        var children = new ListCRUDRepository<Child>(
            new Child { id = 1, parent_id = 1, label = "a" },
            new Child { id = 2, parent_id = 2, label = "b" },
            new Child { id = 3, parent_id = 9, label = "keep" });
        var repo = Repo(parents, children);
        await repo.BulkDelete([new Parent { id = 1 }, new Parent { id = 2 }]);
        Assert.Empty(parents.Items);
        Assert.Equal("keep", Assert.Single(children.Items).label);
    }

    static BulkCRUDRepository<Parent> Repo(
        ListCRUDRepository<Parent> parents,
        ListCRUDRepository<Child> children)
    {
        var repo = new BulkCRUDRepository<Parent>(parents);
        repo.AddEnriched("children", "id", children, "parent_id", many: true);
        return repo;
    }
}
