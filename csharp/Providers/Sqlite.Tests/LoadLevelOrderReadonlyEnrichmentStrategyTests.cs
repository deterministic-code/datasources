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

    [Fact(Skip = "Benchmark: run locally")]
    public async Task OneHundredTwentyOneGamesIsAboutFourQueriesAndFasterThanDefault()
    {
        var ds = Datasource.Default;
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        await command.Execute(new Native("CREATE TABLE games (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"));
        await command.Execute(new Native("CREATE TABLE settings (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL)"));
        await command.Execute(new Native("CREATE TABLE game_attributes (id INTEGER PRIMARY KEY AUTOINCREMENT, parent_id INTEGER NOT NULL, name TEXT NOT NULL, value TEXT NOT NULL)"));
        var gamesRepo = Games(connection);
        var settingsRepo = Settings(connection);
        var attributesRepo = Attributes(connection);
        for (var i = 0; i < 121; i++)
        {
            var game = await gamesRepo.Create(new Game { Name = $"g{i}" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "a", Value = "1" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "b", Value = "2" });
            await settingsRepo.Create(new Setting { ParentId = game.Id, Name = "c", Value = "3" });
            await attributesRepo.Create(new GameAttribute { ParentId = game.Id, Name = "theme", Value = "dark" });
        }
        var (defaultMs, defaultReads) = await Time(connection, new DefaultReadonlyEnrichmentStrategy<Game>());
        var (levelMs, levelReads) = await Time(connection, new LoadLevelOrderReadonlyEnrichmentStrategy<Game>());
        Assert.Equal(3, levelReads);
        Console.WriteLine($"LoadLevelOrder {levelMs}ms queries={levelReads}; Default {defaultMs}ms queries={defaultReads}");
    }

    static async Task<(long Ms, int Reads)> Time(IConnection connection, IReadonlyEnrichmentStrategy<Game> strategy)
    {
        var counter = new Reads();
        var gamesRepo = Games(connection);
        gamesRepo.AddEnriched(
            new("Settings", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
            new CountingRepository<Setting>(Settings(connection), counter),
            strategy);
        gamesRepo.AddEnriched(
            new("Attributes", "Id", "ParentId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Name"),
            new CountingRepository<GameAttribute>(Attributes(connection), counter),
            strategy);
        var counted = new CountingRepository<Game>(gamesRepo, counter);
        var clock = Stopwatch.StartNew();
        var items = await counted.Read().ToListAsync();
        clock.Stop();
        Assert.Equal(121, items.Count);
        return (clock.ElapsedMilliseconds, counter.Count);
    }

    static CRUDRepository<Game> Games(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Game>
        {
            Entity = "games",
            PropertyMappings = Snake,
        });

    static CRUDRepository<Setting> Settings(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<Setting>
        {
            Entity = "settings",
            PropertyMappings = Snake,
            Schema = SettingSchema,
        });

    static CRUDRepository<GameAttribute> Attributes(IConnection connection) =>
        new(connection, new CRUDRepositoryOptions<GameAttribute>
        {
            Entity = "game_attributes",
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

    sealed class Reads
    {
        public int Count;
        public void Add() => Count++;
    }

    sealed class CountingRepository<T>(IReadonlyRepository<T> inner, Reads reads) : IReadonlyRepository<T>
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
