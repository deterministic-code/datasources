using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class LoadLevelOrderReadonlyEnrichmentStrategyTests
{
    sealed class Kind
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
    }

    sealed class Child
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Label { get; set; } = "";
        public List<Note> Notes { get; set; } = [];
    }

    sealed class Note
    {
        public long Id { get; set; }
        public long ChildId { get; set; }
        public string Text { get; set; } = "";
    }

    sealed class Setting
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Name { get; set; } = "";
        public string Value { get; set; } = "";
    }

    sealed class GameAttribute
    {
        public long Id { get; set; }
        public long ParentId { get; set; }
        public string Name { get; set; } = "";
        public string Value { get; set; } = "";
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

    sealed class Mark
    {
        public long Id { get; set; }
        public long PlayId { get; set; }
        public long UserId { get; set; }
        public string Note { get; set; } = "";
    }

    sealed class Seat
    {
        public long Id { get; set; }
        public long PlayId { get; set; }
        public long UserId { get; set; }
        public List<Mark> Marks { get; set; } = [];
    }

    sealed class Game
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
        public Dictionary<string, string> Settings { get; set; } = [];
        public Dictionary<string, string> Attributes { get; set; } = [];
    }

    sealed class Parent
    {
        public long Id { get; set; }
        public long? Kind { get; set; }
        public string KindName { get; set; } = "";
        public List<Child> Children { get; set; } = [];
        public Dictionary<string, string> Settings { get; set; } = [];
        public List<Tag> Tags { get; set; } = [];
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly Schema SettingSchema = new(new Dictionary<string, SchemaColumn>
    {
        ["Id"] = new("integer", IsId: true),
        ["ParentId"] = new("integer"),
        ["Name"] = new("string"),
        ["Value"] = new("string"),
    });

    [Fact]
    public async Task ParityObjectMissingDuplicateAndNullKeys()
    {
        var connection = await OpenConnection();
        var kindsRepo = Open<Kind>(connection);
        var parentsRepo = Open<Parent>(connection);
        var strategy = new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>();
        var person = await kindsRepo.Create(new Kind { Name = "person" });
        await kindsRepo.Create(new Kind { Name = "gone" });
        var kept = await parentsRepo.Create(new Parent { Kind = person.Id });
        var missing = await parentsRepo.Create(new Parent { Kind = 99 });
        var duplicate = await parentsRepo.Create(new Parent { Kind = person.Id });
        await parentsRepo.Create(new Parent { Kind = null });
        var expected = await ReadKinds(connection, new DefaultReadonlyEnrichmentStrategy<Parent>());
        var actual = await ReadKinds(connection, strategy);
        Assert.Equal(expected.Select(row => row.KindName), actual.Select(row => row.KindName));
        Assert.Equal("person", actual.Single(row => row.Id == kept.Id).KindName);
        Assert.Equal("", actual.Single(row => row.Id == missing.Id).KindName);
        Assert.Equal("person", actual.Single(row => row.Id == duplicate.Id).KindName);
        Assert.Equal("", actual.Single(row => row.Kind is null).KindName);
    }

    [Fact]
    public async Task ParityListMissingDuplicateAndNullKeys()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        var shared = await parentsRepo.Create(new Parent());
        var lonely = await parentsRepo.Create(new Parent());
        await parentsRepo.Create(new Parent());
        await childrenRepo.Create(new Child { ParentId = shared.Id, Label = "a" });
        await childrenRepo.Create(new Child { ParentId = shared.Id, Label = "b" });
        await childrenRepo.Create(new Child { ParentId = lonely.Id, Label = "x" });
        var expected = await ReadChildren(connection, new DefaultReadonlyEnrichmentStrategy<Parent>());
        var actual = await ReadChildren(connection, new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        Assert.Equal(
            expected.Select(row => row.Children.Select(child => child.Label).ToArray()),
            actual.Select(row => row.Children.Select(child => child.Label).ToArray()));
    }

    [Fact]
    public async Task ParityDictionaryMissingDuplicateAndNullKeys()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var settingsRepo = Open<Setting>(connection, SettingSchema);
        var shared = await parentsRepo.Create(new Parent());
        var empty = await parentsRepo.Create(new Parent());
        await parentsRepo.Create(new Parent());
        await settingsRepo.Create(new Setting { ParentId = shared.Id, Name = "theme", Value = "dark" });
        await settingsRepo.Create(new Setting { ParentId = shared.Id, Name = "density", Value = "comfortable" });
        var expected = await ReadSettings(connection, new DefaultReadonlyEnrichmentStrategy<Parent>());
        var actual = await ReadSettings(connection, new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        Assert.Equal(expected.Select(SettingsOf), actual.Select(SettingsOf));
        Assert.Equal("dark", actual.Single(row => row.Id == shared.Id).Settings["theme"]);
        Assert.Empty(actual.Single(row => row.Id == empty.Id).Settings);
    }

    [Fact]
    public async Task ParityLinkedMissingDuplicateAndNullKeys()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var tagsRepo = Open<Tag>(connection);
        var linksRepo = Open<ParentTag>(connection);
        var keep = await tagsRepo.Create(new Tag { Name = "keep" });
        var drop = await tagsRepo.Create(new Tag { Name = "drop" });
        var tagged = await parentsRepo.Create(new Parent());
        var other = await parentsRepo.Create(new Parent());
        await parentsRepo.Create(new Parent());
        await linksRepo.Create(new ParentTag { ParentId = tagged.Id, TagId = keep.Id });
        await linksRepo.Create(new ParentTag { ParentId = tagged.Id, TagId = drop.Id });
        await linksRepo.Create(new ParentTag { ParentId = other.Id, TagId = keep.Id });
        var expected = await ReadTags(connection, new DefaultReadonlyEnrichmentStrategy<Parent>());
        var actual = await ReadTags(connection, new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        Assert.Equal(
            expected.Select(row => row.Tags.Select(tag => tag.Name).ToArray()),
            actual.Select(row => row.Tags.Select(tag => tag.Name).ToArray()));
        Assert.Empty(actual.Single(row => row.Id != tagged.Id && row.Id != other.Id).Tags);
    }

    [Fact]
    public async Task OneHundredParentsIsOneReadPerEnrichment()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        for (var i = 0; i < 100; i++)
        {
            var parent = await parentsRepo.Create(new Parent());
            await childrenRepo.Create(new Child { ParentId = parent.Id, Label = $"{i}" });
        }
        var reads = new Reads();
        var counted = new CountingRepository<Child>(childrenRepo, reads);
        parentsRepo.AddEnriched(
            new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
            counted,
            new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal(100, items.Count);
        Assert.Equal(100, items.Sum(row => row.Children.Count));
        Assert.Equal(1, reads.Count);
    }

    [Fact]
    public async Task ZeroQueriesWhenParentsEmpty()
    {
        var connection = await OpenConnection();
        var reads = new Reads();
        var childrenRepo = new CountingRepository<Child>(Open<Child>(connection), reads);
        await new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().Apply(
            childrenRepo,
            new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
            []);
        Assert.Equal(0, reads.Count);
    }

    [Fact]
    public async Task ZeroQueriesWhenKeysNull()
    {
        var connection = await OpenConnection();
        var reads = new Reads();
        var kindsRepo = new CountingRepository<Kind>(Open<Kind>(connection), reads);
        await new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().Apply(
            kindsRepo,
            new("KindName", "Kind", "Id", "Name"),
            [new Parent { Kind = null }, new Parent { Kind = null }]);
        Assert.Equal(0, reads.Count);
    }

    [Fact]
    public async Task ChunksTenKeysByThree()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        for (var i = 0; i < 10; i++)
        {
            var parent = await parentsRepo.Create(new Parent());
            await childrenRepo.Create(new Child { ParentId = parent.Id, Label = $"{i}" });
        }
        var reads = new Reads();
        var counted = new CountingRepository<Child>(childrenRepo, reads);
        parentsRepo.AddEnriched(
            new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
            counted,
            new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>().WithBatchSize(3));
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal(10, items.Sum(row => row.Children.Count));
        Assert.Equal(4, reads.Count);
    }

    [Fact]
    public async Task NestedParentChildGrandchildIsThreeQueries()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var childrenRepo = Open<Child>(connection);
        var notesRepo = Open<Note>(connection);
        var parent = await parentsRepo.Create(new Parent());
        var child = await childrenRepo.Create(new Child { ParentId = parent.Id, Label = "a" });
        await notesRepo.Create(new Note { ChildId = child.Id, Text = "n" });
        var reads = new Reads();
        var countedParents = new CountingRepository<Parent>(parentsRepo, reads);
        var countedChildren = new CountingRepository<Child>(childrenRepo, reads);
        var countedNotes = new CountingRepository<Note>(notesRepo, reads);
        var strategy = new LoadLevelOrderReadonlyEnrichmentStrategy<Child>();
        childrenRepo.AddEnriched(new("Notes", "Id", "ChildId", Type: EnrichmentPropertyType.List), countedNotes, strategy);
        parentsRepo.AddEnriched(
            new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List),
            countedChildren,
            new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        var item = Assert.Single(await countedParents.Read().ToListAsync());
        Assert.Equal("n", Assert.Single(Assert.Single(item.Children).Notes).Text);
        Assert.Equal(3, reads.Count);
    }

    [Fact]
    public async Task CompositeKeysMatchAllPairs()
    {
        var connection = await OpenConnection();
        var seatsRepo = Open<Seat>(connection);
        var marksRepo = Open<Mark>(connection);
        var left = await seatsRepo.Create(new Seat { PlayId = 1, UserId = 10 });
        var right = await seatsRepo.Create(new Seat { PlayId = 1, UserId = 11 });
        await marksRepo.Create(new Mark { PlayId = 1, UserId = 10, Note = "left" });
        await marksRepo.Create(new Mark { PlayId = 1, UserId = 11, Note = "right" });
        var reads = new Reads();
        var counted = new CountingRepository<Mark>(marksRepo, reads);
        seatsRepo.AddEnriched(
            new("Marks", "PlayId", "PlayId", Type: EnrichmentPropertyType.List, Keys: [new("PlayId", "PlayId"), new("UserId", "UserId")]),
            counted,
            new LoadLevelOrderReadonlyEnrichmentStrategy<Seat>());
        var items = await seatsRepo.Read().ToListAsync();
        Assert.Equal("left", Assert.Single(items.Single(row => row.Id == left.Id).Marks).Note);
        Assert.Equal("right", Assert.Single(items.Single(row => row.Id == right.Id).Marks).Note);
        Assert.Equal(1, reads.Count);
    }

    [Fact]
    public async Task LinkedTwoQueriesAndEmptyWhenUnlinked()
    {
        var connection = await OpenConnection();
        var parentsRepo = Open<Parent>(connection);
        var tagsRepo = Open<Tag>(connection);
        var linksRepo = Open<ParentTag>(connection);
        var keep = await tagsRepo.Create(new Tag { Name = "keep" });
        var tagged = await parentsRepo.Create(new Parent());
        var bare = await parentsRepo.Create(new Parent());
        await linksRepo.Create(new ParentTag { ParentId = tagged.Id, TagId = keep.Id });
        var reads = new Reads();
        var countedTags = new CountingRepository<Tag>(tagsRepo, reads);
        var countedLinks = new CountingCRUDRepository<ParentTag>(linksRepo, reads);
        parentsRepo.AddLinked(
            new("Tags", "Id", "Id", Association: new("ParentId", "TagId")),
            countedTags,
            countedLinks,
            new LoadLevelOrderReadonlyEnrichmentStrategy<Parent>());
        var items = await parentsRepo.Read().ToListAsync();
        Assert.Equal("keep", Assert.Single(items.Single(row => row.Id == tagged.Id).Tags).Name);
        Assert.Empty(items.Single(row => row.Id == bare.Id).Tags);
        Assert.Equal(2, reads.Count);
    }

    [Fact]
    public async Task OneHundredTwentyOneGamesThreeSettingsOneAttributeIsThreeQueries()
    {
        var connection = await OpenConnection();
        var gamesRepo = Open<Game>(connection);
        var settingsRepo = Open<Setting>(connection, SettingSchema);
        var attributesRepo = Open<GameAttribute>(connection, SettingSchema);
        for (var i = 0; i < 121; i++)
        {
            var game = await gamesRepo.Create(new Game { Name = $"g{i}" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "a", Value = "1" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "b", Value = "2" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "c", Value = "3" });
            await attributesRepo.Create(new GameAttribute { ParentId = game.Id, Name = "theme", Value = "dark" });
        }
        var reads = new Reads();
        var countedGames = new CountingRepository<Game>(gamesRepo, reads);
        var countedSettings = new CountingRepository<Setting>(settingsRepo, reads);
        var countedAttributes = new CountingRepository<GameAttribute>(attributesRepo, reads);
        var strategy = new LoadLevelOrderReadonlyEnrichmentStrategy<Game>();
        gamesRepo.AddEnriched(
            new("Settings", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
            countedSettings,
            strategy);
        gamesRepo.AddEnriched(
            new("Attributes", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
            countedAttributes,
            strategy);
        var items = await countedGames.Read().ToListAsync();
        Assert.Equal(121, items.Count);
        Assert.All(items, game =>
        {
            Assert.Equal(3, game.Settings.Count);
            Assert.Equal("dark", Assert.Single(game.Attributes).Value);
        });
        Assert.Equal(3, reads.Count);
    }

    static async Task<List<Parent>> ReadKinds(IConnection connection, IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var clone = Open<Parent>(connection);
        clone.AddEnriched(new("KindName", "Kind", "Id", "Name"), Open<Kind>(connection), strategy);
        return await clone.Read().ToListAsync();
    }

    static async Task<List<Parent>> ReadChildren(IConnection connection, IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var clone = Open<Parent>(connection);
        clone.AddEnriched(new("Children", "Id", "ParentId", Type: EnrichmentPropertyType.List), Open<Child>(connection), strategy);
        return await clone.Read().ToListAsync();
    }

    static async Task<List<Parent>> ReadSettings(IConnection connection, IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var clone = Open<Parent>(connection);
        clone.AddEnriched(
            new("Settings", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
            Open<Setting>(connection, SettingSchema),
            strategy);
        return await clone.Read().ToListAsync();
    }

    static async Task<List<Parent>> ReadTags(IConnection connection, IReadonlyEnrichmentStrategy<Parent> strategy)
    {
        var clone = Open<Parent>(connection);
        clone.AddLinked(
            new("Tags", "Id", "Id", Association: new("ParentId", "TagId")),
            Open<Tag>(connection),
            Open<ParentTag>(connection),
            strategy);
        return await clone.Read().ToListAsync();
    }

    static Dictionary<string, string> SettingsOf(Parent parent) =>
        parent.Settings.OrderBy(pair => pair.Key).ToDictionary(pair => pair.Key, pair => pair.Value);

    static CRUDRepository<T> Open<T>(IConnection connection, Schema? schema = null) where T : new() =>
        new(connection, new CRUDRepositoryOptions<T>
        {
            Ids = ["Id"],
            PropertyMappings = Snake,
            Schema = schema,
        });

    static async Task<IConnection> OpenConnection()
    {
        var connection = new MemoryDriver().Connection(null);
        await connection.Open();
        return connection;
    }

    sealed class Reads
    {
        public int Count;
        public void Add() => Count++;
    }

    class CountingRepository<T>(IReadonlyRepository<T> inner, Reads reads) : IReadonlyRepository<T>
    {
        public Type Type => inner.Type;
        public Schema Schema => inner.Schema;
        public bool IsSchemaRequired => inner.IsSchemaRequired;

        public Task<T?> ReadSingle(
            IStatement? query = null,
            IReadOnlyList<Parameter>? parameters = null,
            ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.ReadSingle(query, parameters, options);
        }

        public Task<T?> ReadSingle(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.ReadSingle(where, options);
        }

        public IAsyncEnumerable<T> Read(
            IStatement? query = null,
            IReadOnlyList<Parameter>? parameters = null,
            ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.Read(query, parameters, options);
        }

        public IAsyncEnumerable<T> Read(IWhere where, ReadonlyRepositoryMappingOptions<T>? options = null)
        {
            reads.Add();
            return inner.Read(where, options);
        }
    }

    sealed class CountingCRUDRepository<T>(ICRUDRepository<T> inner, Reads reads) : CountingRepository<T>(inner, reads), ICRUDRepository<T>
        where T : new()
    {
        public IReadOnlyList<string> Ids => inner.Ids;
        public Task<T> Create(T item) => inner.Create(item);
        public Task<Response> Update(T item, IWhere? byWhere = null) => inner.Update(item, byWhere);
        public Task<Response> Delete(T item, IWhere? byWhere = null) => inner.Delete(item, byWhere);
        public Task<T?> ReadSingleById(Value id) => inner.ReadSingleById(id);
        public Task<T?> ReadSingleById(Row ids) => inner.ReadSingleById(ids);
    }
}
