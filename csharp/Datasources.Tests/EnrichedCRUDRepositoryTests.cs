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
        await typesRepo.Create(new TypeRow { name = "person" });
        var type = await typesRepo.ReadSingle();
        Repo(parentsRepo, typesRepo, childrenRepo);
        await parentsRepo.Create(new Parent
        {
            type_id = type!.id,
            children = [new Child { label = "a" }, new Child { label = "b" }]
        });
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        Assert.True(parent.id > 0);
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
        await parentsRepo.Create(new Parent { type_id = 0, children = [new Child { label = "n" }] });
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        Assert.True(parent.id > 0);
        Assert.Equal(parent.id, child.parent_id);
    }

    [Fact]
    public async Task UpdateReplaceSyncsChildren()
    {
        var connection = await Setup();
        var parentsRepo = new CRUDRepository<Parent>(connection);
        var childrenRepo = new CRUDRepository<Child>(connection);
        await parentsRepo.Create(new Parent { type_id = 0 });
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        await childrenRepo.Create(new Child { parent_id = parent.id, label = "b" });
        var kept = (await childrenRepo.Read().ToListAsync()).Single(child => child.label == "a");
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        await parentsRepo.Update(new Parent
        {
            id = parent.id,
            children =
            [
                new Child { id = kept.id, label = "aa" },
                new Child { label = "c" }
            ]
        });
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
        await parentsRepo.Create(new Parent());
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        await notesRepo.Create(new Note { child_id = child.id, text = "n" });
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        await parentsRepo.Update(new Parent
        {
            id = parent.id,
            children = [new Child { label = "fresh" }]
        });
        var created = Assert.Single(await childrenRepo.Read().ToListAsync());
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
        await parentsRepo.Create(new Tagged());
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        await tagsRepo.Create(new Tag { name = "keep" });
        await tagsRepo.Create(new Tag { name = "drop" });
        var tags = await tagsRepo.Read().ToListAsync();
        var keep = tags.Single(tag => tag.name == "keep");
        var drop = tags.Single(tag => tag.name == "drop");
        await linksRepo.Create(new ParentTag { parent_id = parent.id, tag_id = keep.id });
        await linksRepo.Create(new ParentTag { parent_id = parent.id, tag_id = drop.id });
        var oldLink = (await linksRepo.Read().ToListAsync()).Single(link => link.tag_id == keep.id);
        parentsRepo.AddLinked(new("tags", "id", "id", Association: new("parent_id", "tag_id")), tagsRepo, linksRepo);
        await parentsRepo.Update(new Tagged { id = parent.id, tags = [new Tag { id = keep.id }] });
        var link = Assert.Single(await linksRepo.Read().ToListAsync());
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
        await parentsRepo.Create(new Parent());
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        await notesRepo.Create(new Note { child_id = child.id, text = "n" });
        await notesRepo.Create(new Note { text = "other" });
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        await parentsRepo.Delete(new Parent { id = parent.id });
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
        await parentsRepo.Create(new Parent());
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        await childrenRepo.Create(new Child { parent_id = parent.id, label = "a" });
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        await parentsRepo.Delete(new Parent { id = parent.id });
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
        await childTypesRepo.Create(new TypeRow { name = "home" });
        var home = Assert.Single(await childTypesRepo.Read().ToListAsync());
        childrenRepo.AddEnriched(new("type_name", "type_id", "id", "name"), childTypesRepo);
        childrenRepo.AddEnriched(new("notes", "id", "child_id", Many: true), notesRepo);
        parentsRepo.AddEnriched(new("children", "id", "parent_id", Many: true), childrenRepo);
        await parentsRepo.Create(new Parent
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
        var connection = new MemoryProvider().Connection(null);
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
