using Datasources;
using Datasources.Sqlite;
using System.Diagnostics;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class LoadLevelOrderReadonlyEnrichmentStrategyTests
{
    public LoadLevelOrderReadonlyEnrichmentStrategyTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class Game
    {
        public long Id { get; set; }
        public string Name { get; set; } = "";
        public Dictionary<string, string> Settings { get; set; } = [];
        public Dictionary<string, string> Attributes { get; set; } = [];
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

    [Fact]
    public async Task OneHundredTwentyOneGamesReadAndReadSingleByIdAreConstantQueries()
    {
        var connection = await Open();
        var firstId = await Seed(connection);
        var none = await Time(connection, settings: false, attributes: false, readSingleId: null);
        Assert.Equal(121, none.Count);
        Assert.Equal(1, none.Games);
        Assert.Equal(0, none.Settings);
        Assert.Equal(0, none.Attributes);
        var settings = await Time(connection, settings: true, attributes: false, readSingleId: null);
        Assert.Equal(121, settings.Count);
        Assert.Equal(1, settings.Games);
        Assert.Equal(1, settings.Settings);
        Assert.Equal(0, settings.Attributes);
        var full = await Time(connection, settings: true, attributes: true, readSingleId: null);
        Assert.Equal(121, full.Count);
        Assert.Equal(1, full.Games);
        Assert.Equal(1, full.Settings);
        Assert.Equal(1, full.Attributes);
        var skipped = await Time(connection, settings: true, attributes: true, readSingleId: null, EnrichmentSelection.None);
        Assert.Equal(121, skipped.Count);
        Assert.Equal(1, skipped.Games);
        Assert.Equal(0, skipped.Settings);
        Assert.Equal(0, skipped.Attributes);
        var settingsOnly = await Time(connection, settings: true, attributes: true, readSingleId: null, EnrichmentSelection.Only("Settings"));
        Assert.Equal(121, settingsOnly.Count);
        Assert.Equal(1, settingsOnly.Games);
        Assert.Equal(1, settingsOnly.Settings);
        Assert.Equal(0, settingsOnly.Attributes);
        var single = await Time(connection, settings: true, attributes: true, readSingleId: firstId);
        Assert.Equal(1, single.Count);
        Assert.Equal(1, single.Games);
        Assert.Equal(1, single.Settings);
        Assert.Equal(1, single.Attributes);
        var singleSettings = await Time(connection, settings: true, attributes: true, readSingleId: firstId, EnrichmentSelection.Only("Settings"));
        Assert.Equal(1, singleSettings.Count);
        Assert.Equal(1, singleSettings.Games);
        Assert.Equal(1, singleSettings.Settings);
        Assert.Equal(0, singleSettings.Attributes);
        Console.WriteLine(
            $"sqlite Game view: list none={none.Ms}ms q={none.Games}; list settings={settings.Ms}ms q=games{settings.Games}+settings{settings.Settings}; list both={full.Ms}ms q=3; read_single_by_id={single.Ms}ms q=3");
        if (settings.Ms - none.Ms < none.Ms / 10.0)
            Console.WriteLine("dictionary settings on the 121-row list is not a large share of wall time next to the games table read");
        Console.WriteLine("read_single_by_id still runs three sequential queries; dictionary N+1 is not the list cost, and on a single game it is one extra settings round trip");
    }

    static async Task<IConnection> Open()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native("CREATE TABLE games (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"));
        await command.Execute(new Native("CREATE TABLE settings (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL)"));
        await command.Execute(new Native("CREATE TABLE game_attributes (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL)"));
        return connection;
    }

    static async Task<long> Seed(IConnection connection)
    {
        var gamesRepo = Games(connection);
        var settingsRepo = Settings(connection);
        var attributesRepo = Attributes(connection);
        long firstId = 0;
        for (var i = 0; i < 121; i++)
        {
            var game = await gamesRepo.Create(new Game { Name = $"g{i}" });
            if (i == 0) firstId = game.Id;
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "a", Value = "1" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "b", Value = "2" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "c", Value = "3" });
            await attributesRepo.Create(new GameAttribute { ParentId = game.Id, Name = "theme", Value = "dark" });
        }
        return firstId;
    }

    static async Task<Timed> Time(IConnection connection, bool settings, bool attributes, long? readSingleId, EnrichmentSelection? enrich = null)
    {
        var games = new Reads();
        var settingReads = new Reads();
        var attributeReads = new Reads();
        var gamesRepo = Games(connection);
        if (settings)
            gamesRepo.AddEnriched(
                new("Settings", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
                new CountingRepository<Setting>(Settings(connection), settingReads));
        if (attributes)
            gamesRepo.AddEnriched(
                new("Attributes", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
                new CountingRepository<GameAttribute>(Attributes(connection), attributeReads));
        var counted = new CountingCRUDRepository<Game>(gamesRepo, games);
        var clock = Stopwatch.StartNew();
        var count = 0;
        if (readSingleId is { } id)
        {
            var options = enrich is null ? null : new ReadonlyRepositoryMappingOptions<Game> { Enrich = enrich };
            var item = await counted.ReadSingle(new Where(new Operator.Equals("Id", id)), options);
            count = item is null ? 0 : 1;
            if (enrich?.Includes("Settings") != false)
                Assert.Equal(3, item!.Settings.Count);
        }
        else
        {
            var options = enrich is null ? null : new ReadonlyRepositoryMappingOptions<Game> { Enrich = enrich };
            var items = await counted.Read(options: options).ToListAsync();
            count = items.Count;
            if (settings && enrich?.Includes("Settings") != false)
                Assert.All(items, game => Assert.Equal(3, game.Settings.Count));
            if (attributes && enrich?.Includes("Attributes") == true)
                Assert.All(items, game => Assert.Equal("dark", game.Attributes["theme"]));
        }
        clock.Stop();
        return new Timed(clock.Elapsed.TotalMilliseconds, count, games.Count, settingReads.Count, attributeReads.Count);
    }

    static CRUDRepository<Game> Games(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Game>
        {
            Entity = "games",
            Ids = ["Id"],
            PropertyMappings = Snake,
        });

    static CRUDRepository<Setting> Settings(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Setting>
        {
            Entity = "settings",
            Ids = ["Id"],
            PropertyMappings = Snake,
            Schema = SettingSchema,
        });

    static CRUDRepository<GameAttribute> Attributes(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<GameAttribute>
        {
            Entity = "game_attributes",
            Ids = ["Id"],
            PropertyMappings = Snake,
            Schema = SettingSchema,
        });

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly Schema SettingSchema = new(new Dictionary<string, SchemaColumn>
    {
        ["Id"] = new("integer", IsId: true),
        ["ParentId"] = new("integer"),
        ["Name"] = new("string"),
        ["Value"] = new("string"),
    });

    sealed class Timed(double ms, int count, int games, int settings, int attributes)
    {
        public double Ms { get; } = ms;
        public int Count { get; } = count;
        public int Games { get; } = games;
        public int Settings { get; } = settings;
        public int Attributes { get; } = attributes;
    }

    sealed class Reads
    {
        public int Count;
        public void Add() => Count++;
    }

    sealed class CountingCRUDRepository<T> : CountingRepository<T>, ICRUDRepository<T>
        where T : new()
    {
        readonly ICRUDRepository<T> crud;
        readonly Reads counted;

        public CountingCRUDRepository(ICRUDRepository<T> inner, Reads reads) : base(inner, reads)
        {
            crud = inner;
            counted = reads;
        }

        public IReadOnlyList<string> Ids => crud.Ids;
        public Task<T> Create(T item) => crud.Create(item);
        public Task<Response> Update(T item, IWhere? byWhere = null) => crud.Update(item, byWhere);
        public Task<Response> Delete(T item, IWhere? byWhere = null) => crud.Delete(item, byWhere);
        public Task<T?> ReadSingleById(Value id)
        {
            counted.Add();
            return crud.ReadSingleById(id);
        }
        public Task<T?> ReadSingleById(Row ids)
        {
            counted.Add();
            return crud.ReadSingleById(ids);
        }
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
}
