using Datasources;

namespace Datasources.Tests;

public class CRUDRepositoryTests
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
        Repo(parents, types, children);
        await parents.Create(new Parent
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
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        await parents.Create(new Parent { children = [new Child { label = "n" }] });
        Assert.Equal(5, Assert.Single(children.Items).parent_id);
    }

    [Fact]
    public async Task UpdateReplaceSyncsChildren()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var children = new ListCRUDRepository<Child>(
            new Child { id = 1, parent_id = 1, label = "a" },
            new Child { id = 2, parent_id = 1, label = "b" });
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        await parents.Update(new Parent
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
        children.AddEnriched(new("notes", "id", "child_id", Many: true), notes);
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        await parents.Delete(new Parent { id = 1 });
        Assert.Empty(parents.Items);
        Assert.Empty(children.Items);
        Assert.Equal("other", Assert.Single(notes.Items).text);
    }

    [Fact]
    public async Task DeleteStrategyCanSkipChildren()
    {
        var children = new ListCRUDRepository<Child>(
            new Child { id = 3, parent_id = 1, label = "a" });
        var parents = new ListCRUDRepository<Parent>(
            new DefaultWriteEnrichmentStrategy<Parent>(new SkipDelete<Parent>()),
            new Parent { id = 1 });
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        await parents.Delete(new Parent { id = 1 });
        Assert.Empty(parents.Items);
        Assert.Equal("a", Assert.Single(children.Items).label);
    }

    [Fact]
    public async Task NestedManyWritesAndNestedSelectIsSkipped()
    {
        var parents = new ListCRUDRepository<Parent>();
        var children = new ListCRUDRepository<Child>();
        var notes = new ListCRUDRepository<Note>();
        var childTypes = new ListCRUDRepository<TypeRow>(new TypeRow { id = 3, name = "home" });
        children.AddEnriched(new("type_name", "type_id", "id", "name"), childTypes);
        children.AddEnriched(new("notes", "id", "child_id", Many: true), notes);
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        await parents.Create(new Parent
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
        var parents = new ListCRUDRepository<Parent>();
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), new ListReadonlyRepository<Child>());
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            parents.Create(new Parent { children = [new Child { label = "a" }] }));
        Assert.Contains("children", error.Message);
    }

    [Fact]
    public async Task GetStillEnriches()
    {
        var parents = new ListCRUDRepository<Parent>(new Parent { id = 1, type_id = 10 });
        var types = new ListCRUDRepository<TypeRow>(new TypeRow { id = 10, name = "org" });
        var children = new ListCRUDRepository<Child>(new Child { parent_id = 1, label = "a" });
        Repo(parents, types, children);
        var item = await parents.ReadSingle();
        Assert.Equal("org", item!.type_name);
        Assert.Equal("a", Assert.Single(item.children).label);
    }

    sealed class SkipDelete<T> : DeleteEnrichmentStrategy<T>
    {
        public override Task DeleteOwned(
            StoredEnrichment _entry,
            Enrichment _enrichment,
            object _parent,
            Func<IReadOnlyList<object>, Task> _deleteNested) =>
            Task.CompletedTask;

        public override Task DeleteLinked(StoredEnrichment _entry, Enrichment _enrichment, object _parent) =>
            Task.CompletedTask;
    }

    static ListCRUDRepository<Parent> Repo(
        ListCRUDRepository<Parent> parents,
        ListCRUDRepository<TypeRow> types,
        ListCRUDRepository<Child> children)
    {
        parents.AddEnriched(new("type_name", "type_id", "id", "name"), types);
        parents.AddEnriched(new("children", "id", "parent_id", Many: true), children);
        return parents;
    }
}
