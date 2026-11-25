using Datasources;
using Datasources.Memory;

namespace Datasources.Tests;

public class EnrichmentMatchTests
{
    sealed class Mark
    {
        public long Id { get; set; }
        public long PlayId { get; set; }
        public long UserId { get; set; }
        public string Label { get; set; } = "";
    }

    sealed class Setting
    {
        public long Id { get; set; }
        public long PlayId { get; set; }
        public long UserId { get; set; }
        public string Name { get; set; } = "";
        public string Value { get; set; } = "";
    }

    sealed class Seat
    {
        public long Id { get; set; }
        public long PlayId { get; set; }
        public long UserId { get; set; }
        public List<Mark> Marks { get; set; } = [];
        public Dictionary<string, string> Settings { get; set; } = [];
    }

    static readonly ChangeCasingMapper Snake = new(Casing.Pascal, Casing.Snake);

    static readonly Enrichment Marks = new(
        "Marks",
        "PlayId",
        "PlayId",
        Type: EnrichmentPropertyType.List,
        Keys: [new("PlayId", "PlayId"), new("UserId", "UserId")]);

    static readonly Enrichment Settings = new(
        "Settings",
        "PlayId",
        "PlayId",
        "Value",
        EnrichmentPropertyType.Dictionary,
        DictionaryKey: "Name",
        Keys: [new("PlayId", "PlayId"), new("UserId", "UserId")]);

    static readonly Schema SettingSchema = new(new Dictionary<string, SchemaColumn>
    {
        ["Id"] = new("integer", IsId: true),
        ["PlayId"] = new("integer"),
        ["UserId"] = new("integer"),
        ["Name"] = new("string"),
        ["Value"] = new("string"),
    });

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

    static async Task SeedMarks(IConnection seats, IConnection events)
    {
        await seats.CreateCommand().Execute("insert", [new Parameter("play_id", 1), new Parameter("user_id", 7)]);
        await seats.CreateCommand().Execute("insert", [new Parameter("play_id", 1), new Parameter("user_id", 8)]);
        await events.CreateCommand().Execute("insert", [new Parameter("play_id", 1), new Parameter("user_id", 7), new Parameter("label", "mine")]);
        await events.CreateCommand().Execute("insert", [new Parameter("play_id", 1), new Parameter("user_id", 8), new Parameter("label", "theirs")]);
        await events.CreateCommand().Execute("insert", [new Parameter("play_id", 1), new Parameter("user_id", 7), new Parameter("label", "also")]);
    }

    [Fact]
    public async Task ReadsOnlyTheRowsThatMatchEveryColumn()
    {
        var seats = await OpenConnection();
        var events = await OpenConnection();
        await SeedMarks(seats, events);
        var seatsRepo = Open<Seat>(seats);
        seatsRepo.AddEnriched(Marks, Open<Mark>(events));
        var rows = await seatsRepo.Read().ToListAsync();
        Assert.Equal(["also", "mine"], rows.Single(row => row.UserId == 7).Marks.Select(mark => mark.Label).Order().ToArray());
        Assert.Equal(["theirs"], rows.Single(row => row.UserId == 8).Marks.Select(mark => mark.Label).ToArray());
    }

    [Fact]
    public async Task HashsetReadsOnlyTheRowsThatMatchEveryColumn()
    {
        var seats = await OpenConnection();
        var events = await OpenConnection();
        await SeedMarks(seats, events);
        var seatsRepo = Open<Seat>(seats);
        seatsRepo.AddEnriched(Marks, Open<Mark>(events), new HashsetOptimizedReadonlyEnrichmentStrategy<Seat>());
        var rows = await seatsRepo.Read().ToListAsync();
        Assert.Equal(["also", "mine"], rows.Single(row => row.UserId == 7).Marks.Select(mark => mark.Label).Order().ToArray());
        Assert.Equal(["theirs"], rows.Single(row => row.UserId == 8).Marks.Select(mark => mark.Label).ToArray());
    }

    [Fact]
    public async Task CreateStampsEveryKeyAndReadDoesNotMixSharedFirstKey()
    {
        var connection = await OpenConnection();
        var seatsRepo = Open<Seat>(connection);
        var marksRepo = Open<Mark>(connection);
        seatsRepo.AddEnriched(Marks, marksRepo);
        await seatsRepo.Create(new Seat
        {
            PlayId = 1,
            UserId = 7,
            Marks = [new Mark { Label = "mine" }],
        });
        await seatsRepo.Create(new Seat
        {
            PlayId = 1,
            UserId = 8,
            Marks = [new Mark { Label = "theirs" }],
        });
        var stored = await marksRepo.Read().ToListAsync();
        Assert.Equal(2, stored.Count);
        Assert.Equal((1L, 7L, "mine"), stored.Select(mark => (mark.PlayId, mark.UserId, mark.Label)).Single(mark => mark.Label == "mine"));
        Assert.Equal((1L, 8L, "theirs"), stored.Select(mark => (mark.PlayId, mark.UserId, mark.Label)).Single(mark => mark.Label == "theirs"));
        var rows = await seatsRepo.Read().ToListAsync();
        Assert.Equal("mine", Assert.Single(rows.Single(row => row.UserId == 7).Marks).Label);
        Assert.Equal("theirs", Assert.Single(rows.Single(row => row.UserId == 8).Marks).Label);
    }

    [Fact]
    public async Task CreateStampsDictionaryOnEveryKey()
    {
        var connection = await OpenConnection();
        var seatsRepo = Open<Seat>(connection);
        var settingsRepo = Open<Setting>(connection, SettingSchema);
        seatsRepo.AddEnriched(Settings, settingsRepo);
        await seatsRepo.Create(new Seat
        {
            PlayId = 1,
            UserId = 7,
            Settings = { ["color"] = "blue" },
        });
        await seatsRepo.Create(new Seat
        {
            PlayId = 1,
            UserId = 8,
            Settings = { ["color"] = "red" },
        });
        var stored = await settingsRepo.Read().ToListAsync();
        Assert.Equal(2, stored.Count);
        Assert.Contains(stored, row => row.PlayId == 1 && row.UserId == 7 && row.Name == "color" && row.Value == "blue");
        Assert.Contains(stored, row => row.PlayId == 1 && row.UserId == 8 && row.Name == "color" && row.Value == "red");
        var rows = await seatsRepo.Read().ToListAsync();
        Assert.Equal("blue", rows.Single(row => row.UserId == 7).Settings["color"]);
        Assert.Equal("red", rows.Single(row => row.UserId == 8).Settings["color"]);
    }
}
