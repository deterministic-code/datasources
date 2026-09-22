using Datasources;

namespace Datasources.Tests;

public class EnrichedCRUDRepositoryTests
{
    sealed class Parent
    {
        public int id { get; set; }
        public int type_id { get; set; }
        public string type_name { get; set; } = "";
        public List<Child> children { get; set; } = [];
    }

    sealed class TypeRow
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class Child
    {
        public int id { get; set; }
        public int parent_id { get; set; }
        public int type_id { get; set; }
        public string label { get; set; } = "";
        public string type_name { get; set; } = "";
        public List<Note> notes { get; set; } = [];
    }

    sealed class Note
    {
        public int id { get; set; }
        public int child_id { get; set; }
        public string text { get; set; } = "";
    }

    [Fact]
    public async Task CreateWritesChildrenStampsFkAndSkipsSelect()
    {
        var parents = new ListCRUDRepository<Parent>();
        var types = new ListCRUDRepository<TypeRow>(new TypeRow { id = 10, name = "person" });
        var children = new ListCRUDRepository<Child>();
        var repo = Repo(parents, types, children);
        await repo.Create(new Parent
        {
            type_id = 10,
            children = [new Child { label = "a" }, new Child { label = "b" }]
        });
        Assert.Equal(1, parents.Items[0].id);
        Assert.Equal(2, children.Items.Count);
        Assert.Equal([1, 1], children.Items.Select(child => child.parent_id));
        Assert.Equal(["a", "b"], children.Items.Select(child => child.label));
        Assert.Single(types.Items);
        Assert.Equal("person", types.Items[0].name);
    }

    [Fact]
    public async Task CreateAppliesGeneratedParentIdBeforeChildFks()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 4 });
        var children = new ListCRUDRepository<Child>();
        var repo = new EnrichedCRUDRepository<Parent>(parents);
        repo.AddEnriched("children", "id", children, "parent_id", many: true);
        await repo.Create(new Parent { children = [new Child { label = "n" }] });
        Assert.Equal(5, Assert.Single(children.Items).parent_id);
    }

    [Fact]
    public async Task UpdateReplaceSyncsChildren()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var children = new ListCRUDRepository<Child>(
            new Child { id = 1, parent_id = 1, label = "a" },
            new Child { id = 2, parent_id = 1, label = "b" });
        var repo = new EnrichedCRUDRepository<Parent>(parents);
        repo.AddEnriched("children", "id", children, "parent_id", many: true);
        await repo.Update(new Parent
        {
            id = 1,
            children =
            [
                new Child { id = 1, label = "aa" },
                new Child { label = "c" }
            ]
        });
        Assert.Equal(2, children.Items.Count);
        Assert.Equal("aa", children.Items.Single(child => child.id == 1).label);
        Assert.Equal("c", children.Items.Single(child => child.id != 1).label);
        Assert.All(children.Items, child => Assert.Equal(1, child.parent_id));
    }

    [Fact]
    public async Task DeleteCascadesChildrenThenParent()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 1 });
        var children = new ListCRUDRepository<Child>(
            new Child { id = 3, parent_id = 1, label = "a" });
        var notes = new ListCRUDRepository<Note>(
            new Note { id = 8, child_id = 3, text = "n" },
            new Note { id = 9, child_id = 9, text = "other" });
        var repo = new EnrichedCRUDRepository<Parent>(parents);
        repo.AddEnriched("children", "id", children, "parent_id", many: true,
            nest: node => node.Include(notes, "notes", "id", "child_id", many: true));
        await repo.Delete(new Parent { id = 1 });
        Assert.Empty(parents.Items);
        Assert.Empty(children.Items);
        Assert.Equal("other", Assert.Single(notes.Items).text);
    }

    [Fact]
    public async Task NestedManyWritesAndNestedSelectIsSkipped()
    {
        var parents = new ListCRUDRepository<Parent>();
        var children = new ListCRUDRepository<Child>();
        var notes = new ListCRUDRepository<Note>();
        var childTypes = new ListCRUDRepository<TypeRow>(new TypeRow { id = 3, name = "home" });
        var repo = new EnrichedCRUDRepository<Parent>(parents);
        repo.AddEnriched("children", "id", children, "parent_id", many: true,
            nest: node =>
            {
                node.Include(childTypes, "type_name", "type_id", "id", "name");
                node.Include(notes, "notes", "id", "child_id", many: true);
            });
        await repo.Create(new Parent
        {
            children =
            [
                new Child
                {
                    type_id = 3,
                    label = "a",
                    notes = [new Note { text = "hi" }]
                }
            ]
        });
        Assert.Equal("hi", Assert.Single(notes.Items).text);
        Assert.Equal(children.Items[0].id, notes.Items[0].child_id);
        Assert.Equal("home", Assert.Single(childTypes.Items).name);
    }

    [Fact]
    public async Task ManyReadonlyViaThrows()
    {
        var repo = new EnrichedCRUDRepository<Parent>(new ListCRUDRepository<Parent>());
        repo.AddEnriched("children", "id", new ListReadonlyRepository<Child>(), "parent_id", many: true);
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            repo.Create(new Parent { children = [new Child { label = "a" }] }));
        Assert.Contains("children", error.Message);
    }

    [Fact]
    public async Task GetStillEnriches()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var types = new ListCRUDRepository<TypeRow>(new TypeRow { id = 10, name = "org" });
        var children = new ListCRUDRepository<Child>(new Child { parent_id = 1, label = "a" });
        var repo = Repo(parents, types, children);
        var item = await repo.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal("a", Assert.Single(item.children).label);
    }

    static EnrichedCRUDRepository<Parent> Repo(
        ListCRUDRepository<Parent> parents,
        ListCRUDRepository<TypeRow> types,
        ListCRUDRepository<Child> children)
    {
        var repo = new EnrichedCRUDRepository<Parent>(parents);
        repo.AddEnriched("type_name", "type_id", types, "id", "name");
        repo.AddEnriched("children", "id", children, "parent_id", many: true);
        return repo;
    }
}
