using Datasources;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
public class MappingCellRepositoryTests
{
    public MappingCellRepositoryTests()
    {
        DatasourceFactory.Reset();
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Environment.SetEnvironmentVariable("INSTALLED_PROVIDERS", "sqlite");
    }

    sealed class MappingCell
    {
        public long Id { get; set; }
        public string TextToString { get; set; } = "";
        public long TextToInteger { get; set; }
        public decimal TextToDecimal { get; set; }
        public DateTime TextToDatetime { get; set; }
        public byte[] TextToBinary { get; set; } = [];
        public Guid TextToGuid { get; set; }
        public long IntToInteger { get; set; }
        public double IntToFloat { get; set; }
        public decimal IntToDecimal { get; set; }
        public bool IntToBoolean { get; set; }
        public long FloatToInteger { get; set; }
        public double FloatToFloat { get; set; }
        public decimal FloatToDecimal { get; set; }
        public long DecimalToInteger { get; set; }
        public double DecimalToFloat { get; set; }
        public decimal DecimalToDecimal { get; set; }
        public bool BoolToBoolean { get; set; }
        public DateTime DatetimeToDatetime { get; set; }
        public byte[] BinaryToBinary { get; set; } = [];
        public Guid BinaryToGuid { get; set; }
        public string GuidToString { get; set; } = "";
        public Guid GuidToGuid { get; set; }
    }

    static readonly DateTime Opened = new(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
    static readonly Guid Token = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
    static readonly byte[] Blob = [1, 2, 3, 4];
    static readonly ChangeCasingMapper Mapper = new(Datasources.Casing.Pascal, Datasources.Casing.Snake);
    static readonly string[] Properties =
    [
        "Id", "TextToString", "TextToInteger", "TextToDecimal", "TextToDatetime", "TextToBinary", "TextToGuid",
        "IntToInteger", "IntToFloat", "IntToDecimal", "IntToBoolean",
        "FloatToInteger", "FloatToFloat", "FloatToDecimal",
        "DecimalToInteger", "DecimalToFloat", "DecimalToDecimal",
        "BoolToBoolean", "DatetimeToDatetime", "BinaryToBinary", "BinaryToGuid", "GuidToString", "GuidToGuid",
    ];

    [Fact]
    public async Task CreateAndReadRoundTripMappingCellConversions()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = ":memory:" });
        await conn.Open();
        await conn.CreateCommand().Execute(new Native(
            """
            CREATE TABLE mapping_cell (
              id INTEGER PRIMARY KEY,
              text_to_string TEXT,
              text_to_integer TEXT,
              text_to_decimal TEXT,
              text_to_datetime TEXT,
              text_to_binary TEXT,
              text_to_guid TEXT,
              int_to_integer INTEGER,
              int_to_float INTEGER,
              int_to_decimal INTEGER,
              int_to_boolean INTEGER,
              float_to_integer REAL,
              float_to_float REAL,
              float_to_decimal REAL,
              decimal_to_integer NUMERIC,
              decimal_to_float NUMERIC,
              decimal_to_decimal NUMERIC,
              bool_to_boolean INTEGER,
              datetime_to_datetime TEXT,
              binary_to_binary BLOB,
              binary_to_guid BLOB,
              guid_to_string TEXT,
              guid_to_guid TEXT
            )
            """));
        var cells = new CRUDRepository<MappingCell>(conn, new CRUDRepositoryOptions<MappingCell>
        {
            Entity = "MappingCell",
            EntityMapper = Mapper,
            PropertyMappings = Mapper,
            Ids = ["Id"],
            ReadProperties = Properties,
            CreateProperties = Properties,
            UpdateProperties = Properties.Where(name => name != "Id").ToArray(),
            Schema = new(new Dictionary<string, SchemaColumn>
            {
                ["Id"] = new("integer", false, false, true),
                ["TextToString"] = new("string"),
                ["TextToInteger"] = new("integer"),
                ["TextToDecimal"] = new("decimal"),
                ["TextToDatetime"] = new("datetime"),
                ["TextToBinary"] = new("binary"),
                ["TextToGuid"] = new("uuid"),
                ["IntToInteger"] = new("integer"),
                ["IntToFloat"] = new("float"),
                ["IntToDecimal"] = new("decimal"),
                ["IntToBoolean"] = new("boolean"),
                ["FloatToInteger"] = new("integer"),
                ["FloatToFloat"] = new("float"),
                ["FloatToDecimal"] = new("decimal"),
                ["DecimalToInteger"] = new("integer"),
                ["DecimalToFloat"] = new("float"),
                ["DecimalToDecimal"] = new("decimal"),
                ["BoolToBoolean"] = new("boolean"),
                ["DatetimeToDatetime"] = new("datetime"),
                ["BinaryToBinary"] = new("binary"),
                ["BinaryToGuid"] = new("uuid"),
                ["GuidToString"] = new("string"),
                ["GuidToGuid"] = new("uuid"),
            }),
        });
        var created = await cells.Create(Seed(1));
        var read = await cells.ReadSingleById(created.Id);
        Assert.Equal("ada", read!.TextToString);
        Assert.Equal(10, read.TextToInteger);
        Assert.Equal(10.5m, read.TextToDecimal);
        Assert.Equal(Opened, read.TextToDatetime);
        Assert.Equal(Blob, read.TextToBinary);
        Assert.Equal(Token, read.TextToGuid);
        Assert.Equal(10, read.IntToInteger);
        Assert.True(read.IntToBoolean);
        Assert.Equal(10.9, read.FloatToFloat);
        Assert.Equal(10.5m, read.DecimalToDecimal);
        Assert.True(read.BoolToBoolean);
        Assert.Equal(Opened, read.DatetimeToDatetime);
        Assert.Equal(Blob, read.BinaryToBinary);
        Assert.Equal(Token, read.BinaryToGuid);
        Assert.Equal(Token.ToString("D"), read.GuidToString);
        Assert.Equal(Token, read.GuidToGuid);
        var found = await cells.ReadSingle(new Where("Id", eq, created.Id));
        Assert.Equal(created.Id, found!.Id);
        await conn.CreateCommand().Execute(new Native("INSERT INTO mapping_cell (id, text_to_integer) VALUES (9, '10.0')"));
        Assert.Equal(10, (await cells.ReadSingleById(9L))!.TextToInteger);
    }

    static MappingCell Seed(long id) => new()
    {
        Id = id,
        TextToString = "ada",
        TextToInteger = 10,
        TextToDecimal = 10.50m,
        TextToDatetime = Opened,
        TextToBinary = Blob,
        TextToGuid = Token,
        IntToInteger = 10,
        IntToFloat = 10,
        IntToDecimal = 7,
        IntToBoolean = true,
        FloatToInteger = 10,
        FloatToFloat = 10.9,
        FloatToDecimal = 10.5m,
        DecimalToInteger = 10,
        DecimalToFloat = 1.25,
        DecimalToDecimal = 10.50m,
        BoolToBoolean = true,
        DatetimeToDatetime = Opened,
        BinaryToBinary = Blob,
        BinaryToGuid = Token,
        GuidToString = Token.ToString("D"),
        GuidToGuid = Token,
    };
}
