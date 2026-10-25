using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class CRUDRepositoryTests
{
    sealed class UserType
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Pref
    {
        public string Locale { get; set; } = "";
        public string Timezone { get; set; } = "";
    }

    sealed class Setting
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Key { get; set; } = "";
        public string Value { get; set; } = "";
    }

    sealed class PrefRow
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Key { get; set; } = "";
        public string Locale { get; set; } = "";
        public string Timezone { get; set; } = "";
    }

    sealed class Host
    {
        public long Id { get; set; }
        public string Label { get; set; } = "";
        public Dictionary<string, string> Settings { get; set; } = [];
        public Dictionary<string, Pref> Prefs { get; set; } = [];
    }

    sealed class Parent
    {
        public long Id { get; set; }
        public UserType Type { get; set; } = new();
        public string TypeName { get; set; } = "";
        public List<Child> Children { get; set; } = [];
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public UserType Type { get; set; } = new();
        public string TypeName { get; set; } = "";
        public string Label { get; set; } = "";
        public List<Note> Notes { get; set; } = [];
    }

    sealed class Note
    {
        public long Id { get; set; }
        public long ChildId { get; set; }
        public string Text { get; set; } = "";
    }

    sealed class Tagged
    {
        public long Id { get; set; }
        public string Label { get; set; } = "";
        public List<Tag> Tags { get; set; } = [];
    }

    sealed class Tag
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class ParentTag
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public long TagId { get; set; }
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly DictionaryMapper<string> References = new(new Dictionary<string, string>
    {
        ["Type"] = "type_id",
    });

    static readonly IMapper<string, string> Columns = new TransformMapper<string, string>(
        name => References.Map(name) is var mapped && mapped != name ? mapped : Snake.Map(name),
        name => References.ReverseMap(name) is var from && from != name ? from : Snake.ReverseMap(name));

    [Fact]
    public async Task CreateWritesChildrenStampsFkAndSkipsSelect()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        var typesRepo = Open<UserType>(connection);
        var childrenRepo = Open<Child>(connection);
        var type = await typesRepo.Create(new UserType { Name = "person" });
        Assert.True(type.Id > 0);
        Repo(parentsRepo, typesRepo, childrenRepo);
        var created = await parentsRepo.Create(new Parent
        {
            Type = new UserType { Id = type.Id },
            Children = [new Child { Label = "a" }, new Child { Label = "b" }]
        });
        Assert.True(created.Id > 0);
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        Assert.Equal(created.Id, parent.Id);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.All(children, child => Assert.Equal(parent.Id, child.ParentId));
        Assert.Equal(["a", "b"], children.Select(child => child.Label));
        Assert.Equal("person", Assert.Single(await typesRepo.Read().ToListAsync()).Name);
    }

    [Fact]
    public async Task CreateAppliesGeneratedParentIdBeforeChildFks()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var created = await parentsRepo.Create(new Parent { Children = [new Child { Label = "n" }] });
        Assert.True(created.Id > 0);
        var parent = Assert.Single(await parentsRepo.Read().ToListAsync());
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        Assert.Equal(created.Id, parent.Id);
        Assert.Equal(parent.Id, child.ParentId);
    }

    [Fact]
    public async Task RoundTripsStringAndObjectDictionaries()
    {
        var connection = await Setup();
        var hostsRepo = Open<Host>(connection);
        var settingsRepo = Open<Setting>(connection);
        var prefsRepo = Open<PrefRow>(connection);
        hostsRepo.AddEnriched(new("Settings", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary), settingsRepo);
        hostsRepo.AddEnriched(new("Prefs", "Id", "ParentId", Type: EnrichmentPropertyType.Dictionary), prefsRepo);
        var created = await hostsRepo.Create(new Host
        {
            Label = "ada",
            Settings = { ["theme"] = "dark", ["density"] = "comfortable" },
            Prefs =
            {
                ["home"] = new Pref { Locale = "en-GB", Timezone = "Europe/London" },
                ["travel"] = new Pref { Locale = "en-US", Timezone = "America/New_York" },
            },
        });
        Assert.True(created.Id > 0);
        var loaded = await hostsRepo.ReadSingle();
        Assert.Equal(new Dictionary<string, string> { ["theme"] = "dark", ["density"] = "comfortable" }, loaded!.Settings);
        Assert.Equal("en-GB", loaded.Prefs["home"].Locale);
        Assert.Equal("Europe/London", loaded.Prefs["home"].Timezone);
        Assert.Equal("en-US", loaded.Prefs["travel"].Locale);
        await hostsRepo.Update(new Host
        {
            Id = created.Id,
            Label = "ada",
            Settings = { ["theme"] = "light", ["locale"] = "en" },
            Prefs =
            {
                ["home"] = new Pref { Locale = "en-GB", Timezone = "Europe/Paris" },
                ["work"] = new Pref { Locale = "en-US", Timezone = "America/New_York" },
            },
        });
        var updated = await hostsRepo.ReadSingle();
        Assert.Equal(new Dictionary<string, string> { ["theme"] = "light", ["locale"] = "en" }, updated!.Settings);
        Assert.False(updated.Settings.ContainsKey("density"));
        Assert.Equal("Europe/Paris", updated.Prefs["home"].Timezone);
        Assert.False(updated.Prefs.ContainsKey("travel"));
        Assert.Equal("en-US", updated.Prefs["work"].Locale);
    }

    [Fact]
    public async Task UpdateReplaceSyncsChildren()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.Id > 0);
        var kept = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "a" });
        Assert.True(kept.Id > 0);
        var dropped = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "b" });
        Assert.True(dropped.Id > 0);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var updated = await parentsRepo.Update(new Parent
        {
            Id = parent.Id,
            Children =
            [
                new Child { Id = kept.Id, Label = "aa" },
                new Child { Label = "c" }
            ]
        });
        Assert.Equal(1, updated.RowsAffected);
        var children = await childrenRepo.Read().ToListAsync();
        Assert.Equal(2, children.Count);
        Assert.Equal("aa", children.Single(child => child.Id == kept.Id).Label);
        Assert.Equal("c", children.Single(child => child.Id != kept.Id).Label);
        Assert.All(children, child => Assert.Equal(parent.Id, child.ParentId));
    }

    [Fact]
    public async Task CascadeUpdateDeletesChildrenAndRecreates()
    {
        var connection = await Setup();
        var parentsRepo = Open(connection, new DefaultWriteEnrichmentStrategy<Parent>(update: new CascadeDeleteUpdateEnrichmentStrategy<Parent>()));
        var childrenRepo = Open<Child>(connection);
        var notesRepo = Open<Note>(connection);
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.Id > 0);
        var child = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "a" });
        Assert.True(child.Id > 0);
        var other = await childrenRepo.Create(new Child { Label = "other" });
        Assert.True(other.Id > 0);
        var note = await notesRepo.Create(new Note { ChildId = child.Id, Text = "n" });
        Assert.True(note.Id > 0);
        childrenRepo.AddEnriched(new("Notes", "Id", "ChildId", Type: EnrichmentPropertyType.List), notesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var updated = await parentsRepo.Update(new Parent
        {
            Id = parent.Id,
            Children = [new Child { Id = child.Id, Label = "fresh" }]
        });
        Assert.Equal(1, updated.RowsAffected);
        var created = (await childrenRepo.Read().ToListAsync()).Single(row => row.Label == "fresh");
        Assert.Equal(child.Id, created.Id);
        Assert.Equal("fresh", created.Label);
        Assert.Equal(parent.Id, created.ParentId);
        Assert.Empty(await notesRepo.Read().ToListAsync());
    }

    [Fact]
    public async Task CascadeUpdateReplacesLinksAndKeepsRelated()
    {
        var connection = await Setup();
        var parentsRepo = Open(connection, new DefaultWriteEnrichmentStrategy<Tagged>(update: new CascadeDeleteUpdateEnrichmentStrategy<Tagged>()));
        var tagsRepo = Open<Tag>(connection);
        var linksRepo = Open<ParentTag>(connection);
        var parent = await parentsRepo.Create(new Tagged());
        Assert.True(parent.Id > 0);
        var keep = await tagsRepo.Create(new Tag { Name = "keep" });
        Assert.True(keep.Id > 0);
        var drop = await tagsRepo.Create(new Tag { Name = "drop" });
        Assert.True(drop.Id > 0);
        var oldLink = await linksRepo.Create(new ParentTag { ParentId = parent.Id, TagId = keep.Id });
        Assert.True(oldLink.Id > 0);
        var droppedLink = await linksRepo.Create(new ParentTag { ParentId = parent.Id, TagId = drop.Id });
        Assert.True(droppedLink.Id > 0);
        var leftover = await linksRepo.Create(new ParentTag { TagId = keep.Id });
        Assert.True(leftover.Id > 0);
        parentsRepo.AddLinked(new("Tags", "Id", "Id", Association: new("ParentId", "TagId")), tagsRepo, linksRepo);
        var updated = await parentsRepo.Update(new Tagged { Id = parent.Id, Label = "p", Tags = [new Tag { Id = keep.Id }] });
        Assert.Equal(1, updated.RowsAffected);
        var link = Assert.Single(await linksRepo.Read().ToListAsync(), row => row.ParentId == parent.Id);
        Assert.Equal(keep.Id, link.TagId);
        Assert.Equal(parent.Id, link.ParentId);
        Assert.NotEqual(oldLink.Id, link.Id);
        Assert.Equal(2, (await tagsRepo.Read().ToListAsync()).Count);
    }

    [Fact]
    public async Task DeleteCascadesChildrenThenParent()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        var notesRepo = Open<Note>(connection);
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.Id > 0);
        var child = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "a" });
        Assert.True(child.Id > 0);
        var note = await notesRepo.Create(new Note { ChildId = child.Id, Text = "n" });
        Assert.True(note.Id > 0);
        var other = await notesRepo.Create(new Note { Text = "other" });
        Assert.True(other.Id > 0);
        childrenRepo.AddEnriched(new("Notes", "Id", "ChildId", Type: EnrichmentPropertyType.List), notesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var removed = await parentsRepo.Delete(new Parent { Id = parent.Id });
        Assert.Equal(1, removed.RowsAffected);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Empty(await childrenRepo.Read().ToListAsync());
        Assert.Equal("other", Assert.Single(await notesRepo.Read().ToListAsync()).Text);
    }

    [Fact]
    public async Task DeleteStrategyCanSkipChildren()
    {
        var connection = await Setup();
        var childrenRepo = Open<Child>(connection);
        var parentsRepo = Open(connection, new DefaultWriteEnrichmentStrategy<Parent>(new SkipDelete<Parent>()));
        var parent = await parentsRepo.Create(new Parent());
        Assert.True(parent.Id > 0);
        var child = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "a" });
        Assert.True(child.Id > 0);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var removed = await parentsRepo.Delete(new Parent { Id = parent.Id });
        Assert.Equal(1, removed.RowsAffected);
        Assert.Empty(await parentsRepo.Read().ToListAsync());
        Assert.Equal("a", Assert.Single(await childrenRepo.Read().ToListAsync()).Label);
    }

    [Fact]
    public async Task NestedManyWritesAndNestedSelectIsSkipped()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        var notesRepo = Open<Note>(connection);
        var childTypesRepo = Open<UserType>(connection);
        var home = await childTypesRepo.Create(new UserType { Name = "home" });
        Assert.True(home.Id > 0);
        childrenRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), childTypesRepo);
        childrenRepo.AddEnriched(new("Notes", "Id", "ChildId", Type: EnrichmentPropertyType.List), notesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        var created = await parentsRepo.Create(new Parent
        {
            Children =
            [
                new Child
                {
                    Type = new UserType { Id = home.Id },
                    Label = "a",
                    Notes = [new Note { Text = "hi" }]
                }
            ]
        });
        Assert.True(created.Id > 0);
        var note = Assert.Single(await notesRepo.Read().ToListAsync());
        var child = Assert.Single(await childrenRepo.Read().ToListAsync());
        Assert.Equal("hi", note.Text);
        Assert.Equal(child.Id, note.ChildId);
        Assert.Equal("home", Assert.Single(await childTypesRepo.Read().ToListAsync()).Name);
    }

    [Fact]
    public async Task ManyReadonlyViaThrows()
    {
        var connection = await Setup();
        var parentsRepo = Open<Parent>(connection);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), Read<Child>(connection));
        var error = await Assert.ThrowsAsync<DatasourceException>(() =>
            parentsRepo.Create(new Parent { Children = [new Child { Label = "a" }] }));
        Assert.Contains("Children", error.Message);
    }

    [Fact]
    public async Task GetStillEnriches()
    {
        var org = new Row { ["name"] = "org" };
        var typesConnection = await Setup(org);
        var typesRepo = Read<UserType>(typesConnection);
        var parent = new Row { ["type_id"] = org["id"] };
        var connection = await Setup(parent);
        var parentsRepo = Open<Parent>(connection);
        var childrenConnection = await Setup(new Row { ["parent_id"] = parent["id"], ["label"] = "a" });
        var childrenRepo = Read<Child>(childrenConnection);
        Repo(parentsRepo, typesRepo, childrenRepo);
        var item = await parentsRepo.ReadSingle();
        Assert.Equal("org", item!.TypeName);
        Assert.Equal("a", Assert.Single(item.Children).Label);
    }

    sealed class RecordingWrite : IWriteEnrichmentStrategy<Parent>
    {
        public int Calls { get; private set; }
        public Task Persist(IRepositoryTree tree, IList<Parent> items, WriteMode mode)
        {
            Calls++;
            return Task.CompletedTask;
        }
    }

    [Fact]
    public async Task CreateAndUpdateUseCallWriteEnrichment()
    {
        var connection = await Setup();
        var recorded = new RecordingWrite();
        var defaults = new RecordingWrite();
        var parentsRepo = Open(connection, defaults);
        var childrenRepo = Open<Child>(connection);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
        await parentsRepo.Create(new Parent { Children = [new Child { Label = "a" }] });
        Assert.Equal(1, defaults.Calls);
        await parentsRepo.Create(new Parent { Children = [new Child { Label = "b" }] }, recorded);
        Assert.Equal(1, recorded.Calls);
        Assert.Equal(1, defaults.Calls);
        await parentsRepo.Update(new Parent { Id = 1, Children = [] }, writeEnrichment: recorded);
        Assert.Equal(2, recorded.Calls);
        Assert.Equal(1, defaults.Calls);
    }

    sealed class SkipDelete<T> : DeleteEnrichmentStrategy<T>
    {
        public override Task DeleteChildren<TChild>(
            ICRUDRepository<TChild> _writer,
            Enrichment _enrichment,
            T _parent,
            Func<IReadOnlyList<TChild>, Task> _deleteNested) =>
            Task.CompletedTask;

        public override Task DeleteLinked<TChild>(ICRUDRepository<TChild> _writer, Enrichment _enrichment, T _parent) =>
            Task.CompletedTask;
    }

    static void Repo(
        CRUDRepository<Parent> parentsRepo,
        IReadonlyRepository<UserType> typesRepo,
        IReadonlyRepository<Child> childrenRepo)
    {
        parentsRepo.AddEnriched(new("TypeName", "Type", "Id", "Name"), typesRepo);
        parentsRepo.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), childrenRepo);
    }

    static CRUDRepository<T> Open<T>(IConnection connection, IWriteEnrichmentStrategy<T>? write = null) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Ids = ["Id"],
            PropertyMappings = Columns,
            WriteEnrichment = write,
        });

    static ReadonlyRepository<T> Read<T>(IConnection connection) where T : new() =>
        new(connection, new ReadonlyRepositoryMappingOptions<T> { PropertyMappings = Columns });

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
