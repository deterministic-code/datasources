using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class MappingCellConversionsTests
{
    static readonly DateTime Opened = new(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
    static readonly Guid Token = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
    static readonly byte[] Blob = [1, 2, 3, 4];

    static readonly IReadOnlyDictionary<string, string> Columns = new Dictionary<string, string>
    {
        ["Id"] = "integer",
        ["TextToString"] = "string",
        ["TextToInteger"] = "integer",
        ["TextToDecimal"] = "decimal",
        ["TextToDatetime"] = "datetime",
        ["TextToBinary"] = "binary",
        ["TextToGuid"] = "uuid",
        ["IntToInteger"] = "integer",
        ["IntToFloat"] = "float",
        ["IntToDecimal"] = "decimal",
        ["IntToBoolean"] = "boolean",
        ["FloatToInteger"] = "integer",
        ["FloatToFloat"] = "float",
        ["FloatToDecimal"] = "decimal",
        ["DecimalToInteger"] = "integer",
        ["DecimalToFloat"] = "float",
        ["DecimalToDecimal"] = "decimal",
        ["BoolToBoolean"] = "boolean",
        ["DatetimeToDatetime"] = "datetime",
        ["BinaryToBinary"] = "binary",
        ["BinaryToGuid"] = "uuid",
        ["GuidToString"] = "string",
        ["GuidToGuid"] = "uuid",
    };

    static RowTypeConverter Converter() =>
        RowTypeConverter.From(new SqliteDriver().RowTypeConverters, Columns);

    [Fact]
    public void ConvertsMappingCellDriverCells()
    {
        var row = Converter().Convert(new Row
        {
            ["Id"] = 1L,
            ["TextToString"] = "ada",
            ["TextToInteger"] = "10",
            ["TextToDecimal"] = "10.50",
            ["TextToDatetime"] = "2020-01-02T03:04:05.000Z",
            ["TextToBinary"] = Convert.ToBase64String(Blob),
            ["TextToGuid"] = Token.ToString("D"),
            ["IntToInteger"] = 10L,
            ["IntToFloat"] = 10L,
            ["IntToDecimal"] = 7L,
            ["IntToBoolean"] = 1L,
            ["FloatToInteger"] = 10.0,
            ["FloatToFloat"] = 10.9,
            ["FloatToDecimal"] = 10.5,
            ["DecimalToDecimal"] = "10.50000000000000",
            ["BoolToBoolean"] = 1L,
            ["DatetimeToDatetime"] = Opened,
            ["BinaryToBinary"] = Blob,
            ["BinaryToGuid"] = Token.ToByteArray(),
            ["GuidToString"] = Token.ToString("D"),
            ["GuidToGuid"] = Token,
        });
        Assert.Equal(new Value.Int(10), row["TextToInteger"]);
        Assert.Equal(new Value.Text("10.5"), row["TextToDecimal"]);
        Assert.Equal(new Value.DateTime(Opened), row["TextToDatetime"]);
        Assert.Equal(new Value.Text(Convert.ToBase64String(Blob)), row["TextToBinary"]);
        Assert.Equal(new Value.Int(10), row["FloatToInteger"]);
        Assert.Equal(new Value.Text("10.5"), row["DecimalToDecimal"]);
        Assert.Equal(new Value.Bool(true), row["IntToBoolean"]);
        Assert.Equal(new Value.DateTime(Opened), row["DatetimeToDatetime"]);
        Assert.Equal(new Value.Text(Token.ToString("D")), row["BinaryToGuid"]);
        Assert.Equal(new Value.Text(Token.ToString("D")), row["GuidToGuid"]);
    }

    [Fact]
    public void RejectsDottedIntegerTextOnMappingCell()
    {
        var converter = Converter();
        Assert.Equal(new Value.Int(10), converter.Convert(new Row { ["TextToInteger"] = "10.0" })["TextToInteger"]);
        Assert.Equal(new Value.Int(10), converter.Convert(new Row { ["FloatToInteger"] = "10.0" })["FloatToInteger"]);
        Assert.Equal(
            new Value.Int(10),
            converter.Convert(new Row { ["DecimalToInteger"] = "10.00000000000000" })["DecimalToInteger"]);
        Assert.Throws<FormatException>(() => converter.Convert(new Row { ["DecimalToInteger"] = "10.50000000000000" }));
    }

    [Fact]
    public void ReverseConvertKeepsIntegerAndIsoDatetime()
    {
        var stored = Converter().ReverseConvert(new Row
        {
            ["IntToInteger"] = 10L,
            ["DatetimeToDatetime"] = Opened,
            ["DecimalToDecimal"] = 10.50m,
        });
        Assert.Equal(new Value.Int(10), stored["IntToInteger"]);
        Assert.Equal(new Value.Text("2020-01-02T03:04:05.000Z"), stored["DatetimeToDatetime"]);
        Assert.Equal(new Value.Text("10.5"), stored["DecimalToDecimal"]);
    }
}
