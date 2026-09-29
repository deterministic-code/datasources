using Datasources;
using Datasources.Memory;

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

    sealed class Tagged
    {
        public int id { get; set; }
        public string label { get; set; } = "";
        public List<Tag> tags { get; set; } = [];
    }

    sealed class Tag
    {
        public int id { get; set; }
        public string name { get; set; } = "";
    }

    sealed class ParentTag
    {
        public int id { get; set; }
        public int parent_id { get; set; }
        public int tag_id { get; set; }
    }

    [Fact]
    public async Task CreateWritesChildrenStampsFkAndSkipsSelect()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var typesRepo = new CRUDRepository<TypeRow>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        var type = await typesRepo.Create(new TypeRow { name = "person" });
        Assert.True(type.id > 0);
        Repo(parentsRepo, typesRepo, childrenRepo);
        var created = await parentsRepo.Create(new Parent
        {
            type_id = type.id,
            children = [new Child { label = "a" }, new Child { label = "b" }]
        });
        Assert.True(created.id > 0);
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        Assert.Equal(created.id, parent.id);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.All(children, child => Assert.Equal(parent.id, child.parent_id));
        Assert.Equal(["a", "b"], children.Select(child => child.label));
        Assert.Equal("person", Assert.Single(await typesRepo.Read().ToListAsync()).name);
    }

    [Fact]
    public async Task CreateAppliesGeneratedParentIdBeforeChildFks()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var created = await parentsRepo.Create(new Parent { type_id = 0, children = [new Child { label = "n" }] });
        Assert.True(created.id > 0);
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        Assert.Equal(created.id, parent.id);
        Assert.Equal(parent.id, child.parent_id);
    }

    [Fact]
    public async Task UpdateReplaceSyncsChildren()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        var parent = await parentsRepo.Create(new Parent { type_id = 0 });
        Assert.True(parent.id > 0);
        var kept = await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        Assert.True(kept.id > 0);
        var dropped = await childrenRepo.Create(new Child { parent_id = parent.id, label = "b" });
        Assert.True(dropped.id > 0);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var updated = await parentsRepo.Update(new Parent
        {
            id = parent.id,
            children =
            [
                new Child { id = kept.id, label = "aa" },
                new Child { label = "c" }
            ]
        });
        Assert.Equal(1, updated.RowsAffected);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.Equal("aa", children.Single(child => child.id == kept.id).label);
        Assert.Equal("c", children.Single(child => child.id != kept.id).label);
        Assert.All(children, child => Assert.Equal(parent.id, child.parent_id));
    }

    [Fact]
    public async Task CascadeUpdateDeletesChildrenAndRecreates()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
        {
            Write = new DefaultWriteEnrichmentStrategy<Parent>(update: new CascadeDeleteUpdateEnrichmentStrategy<Parent>()),
        });
        var childrenRepo = new CRUDRepository<Child>(connection);
        var notesRepo = new CRUDRepository<Note>(connection);
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.id > 0);
        var child = await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        Assert.True(child.id > 0);
        var other = await childrenRepo.Create(new Child { label = "other" });
        Assert.True(other.id > 0);
        var note = await notesRepo.Create(new Note { child_id = child.id, text = "n" });
        Assert.True(note.id > 0);
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var updated = await parentsRepo.Update(new Parent
        {
            id = parent.id,
            children = [new Child { id = child.id, label = "fresh" }]
        });
        Assert.Equal(1, updated.RowsAffected);
        var created = (await childrenRepo.Read().ToListAsync()).Single(row => row.label == "fresh");
        Assert.NotEqual(child.id, created.id);
        Assert.Equal("fresh", created.label);
        Assert.Equal(parent.id, created.parent_id);
        Assert.Empty(await notesRepo.Read().ToListAsync());
    }

    [Fact]
    public async Task CascadeUpdateReplacesLinksAndKeepsRelated()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Tagged>(connection, new CRUDRepositoryOptions<Tagged>
        {
            Write = new DefaultWriteEnrichmentStrategy<Tagged>(update: new CascadeDeleteUpdateEnrichmentStrategy<Tagged>()),
        });
        var tagsRepo = new CRUDRepository<Tag>(connection);
        var linksRepo = new CRUDRepository<ParentTag>(connection);
        var parent = await parentsRepo.Create(new Tagged());
        Assert.True(parent.id > 0);
        var keep = await tagsRepo.Create(new Tag { name = "keep" });
        Assert.True(keep.id > 0);
        var drop = await tagsRepo.Create(new Tag { name = "drop" });
        Assert.True(drop.id > 0);
        var oldLink = await linksRepo.Create(new ParentTag { parent_id = parent.id, tag_id = keep.id });
        Assert.True(oldLink.id > 0);
        var droppedLink = await linksRepo.Create(new ParentTag { parent_id = parent.id, tag_id = drop.id });
        Assert.True(droppedLink.id > 0);
        var leftover = await linksRepo.Create(new ParentTag { tag_id = keep.id });
        Assert.True(leftover.id > 0);
        parentsRepo.AddLinked(new("tags", "id", "id", Association: new("parent_id", "tag_id")), tagsRepo, linksRepo);
        var updated = await parentsRepo.Update(new Tagged { id = parent.id, label = "p", tags = [new Tag { id = keep.id }] });
        Assert.Equal(1, updated.RowsAffected);
        var link = Assert.Single(await linksRepo.Read().ToListAsync(), row => row.parent_id == parent.id);
        Assert.Equal(keep.id, link.tag_id);
        Assert.Equal(parent.id, link.parent_id);
        Assert.NotEqual(oldLink.id, link.id);
        Assert.Equal(2, (await tagsRepo.Read().ToListAsync()).Count);
    }

    [Fact]
    public async Task DeleteCascadesChildrenThenParent()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        var notesRepo = new CRUDRepository<Note>(connection);
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.id > 0);
        var child = await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        Assert.True(child.id > 0);
        var note = await notesRepo.Create(new Note { child_id = child.id, text = "n" });
        Assert.True(note.id > 0);
        var other = await notesRepo.Create(new Note { text = "other" });
        Assert.True(other.id > 0);
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var removed = await parentsRepo.Delete(new Parent { id = parent.id });
        Assert.Equal(1, removed.RowsAffected);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Empty(await childrenRepo.Read().ToListAsync());
        Assert.Equal("other", Assert.Single(await notesRepo.Read().ToListAsync()).text);
    }

    [Fact]
    public async Task DeleteStrategyCanSkipChildren()
    {
        var connection = await Setup();
        var childrenRepo = new CRUDRepository<Child>(connection);
        var parentsRepo = new CRUDRepository<Parent>(connection, new CRUDRepositoryOptions<Parent>
        {
            Write = new DefaultWriteEnrichmentStrategy<Parent>(new SkipDelete<Parent>()),
        });
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.id > 0);
        var child = await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        Assert.True(child.id > 0);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var removed = await parentsRepo.Delete(new Parent { id = parent.id });
        Assert.Equal(1, removed.RowsAffected);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Equal("a", Assert.Single(await childrenRepo.Read().ToListAsync()).label);
    }

    [Fact]
    public async Task NestedManyWritesAndNestedSelectIsSkipped()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        var notesRepo = new CRUDRepository<Note>(connection);
        var childTypesRepo = new CRUDRepository<TypeRow>(connection);
        var home = await childTypesRepo.Create(new TypeRow { name = "home" });
        Assert.True(home.id > 0);
        childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        var created = await parentsRepo.Create(new Parent
        {
            children =
            [
                new Child
                {
                    type_id = home.id,
                    label = "a",
                    notes = [new Note { text = "hi" }]
                }
            ]
        });
        Assert.True(created.id > 0);
        var note = Assert.Single(await notesRepo.Read().ToListAsync());
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        Assert.Equal("hi", note.text);
        Assert.Equal(child.id, note.child_id);
        Assert.Equal("home", Assert.Single(await childTypesRepo.Read().ToListAsync()).name);
    }

    [Fact]
    public async Task ManyReadonlyViaThrows()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), new ReadonlyRepository<Child>(connection));
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            parentsRepo.Create(new Parent { children = [new Child { label = "a" }] }));
        Assert.Contains("children", error.Message);
    }

    [Fact]
    public async Task GetStillEnriches()
    {
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var typesRepo = new ReadonlyRepository<TypeRow>(typesConnection);
        var parent = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent);
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenConnection = await Setup(new Row { ["parent_id"] = parent["id"], ["label"] = "a" });
        var childrenRepo = new ReadonlyRepository<Child>(childrenConnection);
        Repo(parentsRepo, typesRepo, childrenRepo);
        var item = await parentsRepo.ReadSingle();
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

    static void Repo(
        CRUDRepository<Parent> parentsRepo,
        IReadonlyRepository<TypeRow> typesRepo,
        IReadonlyRepository<Child> childrenRepo)
    {
        parentsRepo.AddEnriched(new("type_name", "type_id", "id", "name"), typesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
    }

    static async Task<IConnection> Setup(params Row[] rows)
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        foreach (var row in rows)
        {
            var response = await connection.CreateCommand().Execute(
                "insert",
                [.. row.Select(pair => new Parameter(pair.Key, pair.Value))]);
            row["id"] = response.Result;
        }
        return connection;
    }
}
