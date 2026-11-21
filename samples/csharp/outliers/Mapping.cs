using Datasources;
using static Datasources.Cmp;

namespace Outliers;

public static class Mapping
{
    static readonly Guid Token = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
    static readonly DateTime Opened = new(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
    static readonly byte[] Blob = [1, 2, 3, 4];
    static readonly ChangeCasingMapper Casing = new(Datasources.Casing.Pascal, Datasources.Casing.Snake);
    static readonly string[] Properties =
    [
        "Id", "TextToString", "TextToInteger", "TextToDecimal", "TextToDatetime", "TextToBinary", "TextToGuid",
        "IntToInteger", "IntToFloat", "IntToDecimal", "IntToBoolean",
        "FloatToInteger", "FloatToFloat", "FloatToDecimal",
        "DecimalToInteger", "DecimalToFloat", "DecimalToDecimal",
        "BoolToBoolean", "DatetimeToDatetime", "BinaryToBinary", "BinaryToGuid", "GuidToString", "GuidToGuid",
    ];

    public static async Task<string[]> Run(IConnection connection)
    {
        var lines = new List<string>();
        try
        {
            await connection.CreateCommand().Execute(CreateTable(connection.Provider));
        }
        catch (Exception error)
        {
            return Format.MapError("create_table", error.Message).ToArray();
        }
        var cells = new CRUDRepository<MappingCell>(connection, Options());
        var first = Seed(1);
        var created = await Capture(lines, "create", async () => await cells.Create(first), first);
        await CaptureMany(lines, "read", () => cells.Read());
        await CaptureMany(lines, "read_where", () => cells.Read(new Where("Id", eq, created.Id)));
        await Capture(lines, "read_single", () => cells.ReadSingle(), created);
        await Capture(lines, "read_single_where", () => cells.ReadSingle(new Where("Id", eq, created.Id)), created);
        await Capture(lines, "read_single_by_id", () => cells.ReadSingleById(created.Id), created);
        await Capture(lines, "read_single_by_id_row", () => cells.ReadSingleById(new Row { ["Id"] = created.Id }), created);
        created.TextToString = "grace";
        await CaptureWrite(lines, "update", () => cells.Update(created), () => cells.ReadSingleById(created.Id));
        created.TextToString = "lin";
        await CaptureWrite(lines, "update_where", () => cells.Update(created, new Where("Id", eq, created.Id)), () => cells.ReadSingleById(created.Id));
        var second = Seed(2);
        second.TextToString = "second";
        var extra = await Try(async () => await cells.Create(second), second);
        await CaptureWrite(lines, "delete", () => cells.Delete(extra), () => Task.FromResult<MappingCell?>(extra));
        await CaptureWrite(lines, "delete_where", () => cells.Delete(created, new Where("Id", eq, created.Id)), () => Task.FromResult<MappingCell?>(created));
        await Capture(lines, "create_again", async () => await cells.Create(Seed(3)), Seed(3));
        return [.. lines];
    }

    static CRUDRepositoryOptions<MappingCell> Options() => new()
    {
        Entity = "MappingCell",
        EntityMapper = Casing,
        PropertyMappings = Casing,
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
    };

    static CreateTableStatement CreateTable(string provider)
    {
        var text = Native(provider, "text");
        var integer = Native(provider, "integer");
        var floating = Native(provider, "float");
        var decimalType = Native(provider, "decimal");
        var boolean = Native(provider, "boolean");
        var datetime = Native(provider, "datetime");
        var binary = Native(provider, "binary");
        var guid = Native(provider, "guid");
        return new("mapping_cell",
        [
            new Column("id", integer, Identity: true),
            new Column("text_to_string", text),
            new Column("text_to_integer", text),
            new Column("text_to_decimal", text),
            new Column("text_to_datetime", text),
            new Column("text_to_binary", text),
            new Column("text_to_guid", text),
            new Column("int_to_integer", integer),
            new Column("int_to_float", integer),
            new Column("int_to_decimal", integer),
            new Column("int_to_boolean", integer),
            new Column("float_to_integer", floating),
            new Column("float_to_float", floating),
            new Column("float_to_decimal", floating),
            new Column("decimal_to_integer", decimalType),
            new Column("decimal_to_float", decimalType),
            new Column("decimal_to_decimal", decimalType),
            new Column("bool_to_boolean", boolean),
            new Column("datetime_to_datetime", datetime),
            new Column("binary_to_binary", binary),
            new Column("binary_to_guid", binary),
            new Column("guid_to_string", guid),
            new Column("guid_to_guid", guid),
        ],
        provider == "sqlite" ? [] : [new PrimaryKey("pk_mapping_cell", ["id"])]);
    }

    static string Native(string provider, string source) => (provider, source) switch
    {
        ("mysql", "text") => "TEXT",
        ("mysql", "integer") => "INT",
        ("mysql", "float") => "DOUBLE",
        ("mysql", "decimal") => "DECIMAL(28, 14)",
        ("mysql", "boolean") => "TINYINT(1)",
        ("mysql", "datetime") => "DATETIME(3)",
        ("mysql", "binary") => "BLOB",
        ("mysql", "guid") => "CHAR(36)",
        ("postgres", "text") => "TEXT",
        ("postgres", "integer") => "INTEGER",
        ("postgres", "float") => "DOUBLE PRECISION",
        ("postgres", "decimal") => "NUMERIC",
        ("postgres", "boolean") => "BOOLEAN",
        ("postgres", "datetime") => "TIMESTAMPTZ",
        ("postgres", "binary") => "BYTEA",
        ("postgres", "guid") => "UUID",
        (_, "text") => "TEXT",
        (_, "integer") => "INTEGER",
        (_, "float") => "REAL",
        (_, "decimal") => "NUMERIC",
        (_, "boolean") => "BOOLEAN",
        (_, "datetime") => "TEXT",
        (_, "binary") => "BLOB",
        _ => "TEXT",
    };

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
        IntToFloat = 9007199254740993,
        IntToDecimal = 7,
        IntToBoolean = true,
        FloatToInteger = 10,
        FloatToFloat = 10.9,
        FloatToDecimal = 10.5m,
        DecimalToInteger = 10,
        DecimalToFloat = 1.2345678901234567,
        DecimalToDecimal = 10.50m,
        BoolToBoolean = true,
        DatetimeToDatetime = Opened,
        BinaryToBinary = Blob,
        BinaryToGuid = Token,
        GuidToString = Token.ToString("D"),
        GuidToGuid = Token,
    };

    static async Task<MappingCell> Capture(
        List<string> lines,
        string method,
        Func<Task<MappingCell?>> call,
        MappingCell fallback)
    {
        try
        {
            var row = await call() ?? fallback;
            lines.AddRange(Format.MapLines(method, row));
            return row;
        }
        catch (Exception error)
        {
            lines.AddRange(Format.MapError(method, error.Message));
            return fallback;
        }
    }

    static async Task CaptureMany(List<string> lines, string method, Func<IAsyncEnumerable<MappingCell>> call)
    {
        try
        {
            await foreach (var row in call())
                lines.AddRange(Format.MapLines(method, row));
        }
        catch (Exception error)
        {
            lines.AddRange(Format.MapError(method, error.Message));
        }
    }

    static async Task CaptureWrite(
        List<string> lines,
        string method,
        Func<Task<Response>> call,
        Func<Task<MappingCell?>> read)
    {
        try
        {
            await call();
            var row = await read();
            if (row is null) lines.Add($"map {method}");
            else lines.AddRange(Format.MapLines(method, row));
        }
        catch (Exception error)
        {
            lines.AddRange(Format.MapError(method, error.Message));
        }
    }

    static async Task<MappingCell> Try(Func<Task<MappingCell>> call, MappingCell fallback)
    {
        try
        {
            return await call();
        }
        catch
        {
            return fallback;
        }
    }
}
