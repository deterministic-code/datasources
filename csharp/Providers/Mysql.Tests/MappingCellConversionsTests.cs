using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MappingCellConversionsTests
{
    static readonly DateTime Opened = new(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);

    static readonly IReadOnlyDictionary<string, string> Columns = new Dictionary<string, string>
    {
        ["TextToInteger"] = "integer",
        ["TextToDecimal"] = "decimal",
        ["TextToDatetime"] = "datetime",
        ["TextToBinary"] = "binary",
        ["FloatToInteger"] = "integer",
        ["DecimalToInteger"] = "integer",
        ["DecimalToDecimal"] = "decimal",
        ["DatetimeToDatetime"] = "datetime",
        ["IntToBoolean"] = "boolean",
    };

    static RowTypeConverter Converter() =>
        RowTypeConverter.From(new MysqlDriver().RowTypeConverters, Columns);

    [Fact]
    public void ConvertsMappingCellDriverCells()
    {
        var row = Converter().Convert(new Row
        {
            ["TextToInteger"] = "10",
            ["TextToDecimal"] = "10.50",
            ["TextToDatetime"] = "2020-01-02T03:04:05.000Z",
            ["TextToBinary"] = System.Text.Encoding.Latin1.GetString(new byte[] { 1, 2, 3, 4 }),
            ["FloatToInteger"] = 10.0,
            ["DecimalToDecimal"] = "10.50000000000000",
            ["DatetimeToDatetime"] = Opened,
            ["IntToBoolean"] = 1L,
        });
        Assert.Equal(new Value.Int(10), row["TextToInteger"]);
        Assert.Equal(new Value.Text("10.5"), row["TextToDecimal"]);
        Assert.Equal(new Value.DateTime(Opened), row["TextToDatetime"]);
        Assert.Equal(new byte[] { 1, 2, 3, 4 }, row["TextToBinary"].ToObject());
        Assert.Equal(new Value.Int(10), row["FloatToInteger"]);
        Assert.Equal(new Value.Text("10.5"), row["DecimalToDecimal"]);
        Assert.Equal(new Value.DateTime(Opened), row["DatetimeToDatetime"]);
        Assert.Equal(new Value.Bool(true), row["IntToBoolean"]);
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
    public void ReverseConvertWritesMysqlDatetimeStamp()
    {
        var stored = Converter().ReverseConvert(new Row
        {
            ["IntToInteger"] = 10L,
            ["DatetimeToDatetime"] = Opened,
            ["DecimalToDecimal"] = 10.50m,
        });
        Assert.Equal(new Value.Text("2020-01-02 03:04:05.000"), stored["DatetimeToDatetime"]);
        Assert.Equal(new Value.Text("10.5"), stored["DecimalToDecimal"]);
    }

    [Fact]
    public void ConvertsAndReversesTheMappingCellMatrix()
    {
        var token = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        var blob = new byte[] { 1, 2, 3, 4 };
        var columns = new Dictionary<string, string>
        {
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
        var converter = RowTypeConverter.From(new MysqlDriver().RowTypeConverters, columns);
        var row = converter.Convert(new Row
        {
            ["TextToString"] = "ada",
            ["TextToInteger"] = "10",
            ["TextToDecimal"] = "10.50",
            ["TextToDatetime"] = "2020-01-02T03:04:05.000Z",
            ["TextToBinary"] = System.Text.Encoding.Latin1.GetString(blob),
            ["TextToGuid"] = token.ToString("D"),
            ["IntToInteger"] = 10L,
            ["IntToFloat"] = 10L,
            ["IntToDecimal"] = 7L,
            ["IntToBoolean"] = 1L,
            ["FloatToInteger"] = 10.0,
            ["FloatToFloat"] = 10.9,
            ["FloatToDecimal"] = 10.5,
            ["DecimalToInteger"] = 10.00000000000000m,
            ["DecimalToFloat"] = 1.2345678901234567m,
            ["DecimalToDecimal"] = 10.50000000000000m,
            ["BoolToBoolean"] = 1L,
            ["DatetimeToDatetime"] = Opened,
            ["BinaryToBinary"] = blob,
            ["BinaryToGuid"] = System.Text.Encoding.ASCII.GetBytes(token.ToString("D")),
            ["GuidToString"] = token.ToString("D"),
            ["GuidToGuid"] = token,
        });
        Assert.Equal(new Value.Text("ada"), row["TextToString"]);
        Assert.Equal(new Value.Int(10), row["TextToInteger"]);
        Assert.Equal(new Value.Text("10.5"), row["TextToDecimal"]);
        Assert.Equal(new Value.DateTime(Opened), row["TextToDatetime"]);
        Assert.Equal(blob, row["TextToBinary"].ToObject());
        Assert.Equal(new Value.Text(token.ToString("D")), row["TextToGuid"]);
        Assert.Equal(new Value.Int(10), row["IntToInteger"]);
        Assert.Equal(new Value.Float(10), row["IntToFloat"]);
        Assert.Equal(new Value.Text("7"), row["IntToDecimal"]);
        Assert.Equal(new Value.Bool(true), row["IntToBoolean"]);
        Assert.Equal(new Value.Int(10), row["FloatToInteger"]);
        Assert.Equal(new Value.Float(10.9), row["FloatToFloat"]);
        Assert.Equal(new Value.Text("10.5"), row["FloatToDecimal"]);
        Assert.Equal(new Value.Int(10), row["DecimalToInteger"]);
        Assert.Equal(new Value.Float((double)1.2345678901234567m), row["DecimalToFloat"]);
        Assert.Equal(new Value.Text("10.5"), row["DecimalToDecimal"]);
        Assert.Equal(new Value.Bool(true), row["BoolToBoolean"]);
        Assert.Equal(new Value.DateTime(Opened), row["DatetimeToDatetime"]);
        Assert.Equal(blob, row["BinaryToBinary"].ToObject());
        Assert.Equal(new Value.Text(token.ToString("D")), row["BinaryToGuid"]);
        Assert.Equal(new Value.Text(token.ToString("D")), row["GuidToString"]);
        Assert.Equal(new Value.Text(token.ToString("D")), row["GuidToGuid"]);
        Assert.Equal(
            new Value.Text(token.ToString("D")),
            converter.Convert(new Row { ["BinaryToGuid"] = token.ToByteArray() })["BinaryToGuid"]);
        Assert.Throws<FormatException>(() => converter.Convert(new Row { ["DecimalToInteger"] = 10.5m }));
        var stored = converter.ReverseConvert(new Row
        {
            ["IntToInteger"] = 10L,
            ["IntToFloat"] = 10.0,
            ["IntToBoolean"] = true,
            ["FloatToInteger"] = 10L,
            ["FloatToDecimal"] = 10.5m,
            ["DecimalToInteger"] = 10L,
            ["DecimalToFloat"] = 1.2345678901234567,
            ["DecimalToDecimal"] = 10.50m,
            ["BoolToBoolean"] = false,
            ["DatetimeToDatetime"] = Opened,
            ["BinaryToGuid"] = token,
            ["GuidToGuid"] = token,
        });
        Assert.Equal(new Value.Int(10), stored["IntToInteger"]);
        Assert.Equal(new Value.Float(10), stored["IntToFloat"]);
        Assert.Equal(new Value.Int(1), stored["IntToBoolean"]);
        Assert.Equal(new Value.Int(10), stored["FloatToInteger"]);
        Assert.Equal(new Value.Text("10.5"), stored["FloatToDecimal"]);
        Assert.Equal(new Value.Int(10), stored["DecimalToInteger"]);
        Assert.Equal(new Value.Float(1.2345678901234567), stored["DecimalToFloat"]);
        Assert.Equal(new Value.Text("10.5"), stored["DecimalToDecimal"]);
        Assert.Equal(new Value.Int(0), stored["BoolToBoolean"]);
        Assert.Equal(new Value.Text("2020-01-02 03:04:05.000"), stored["DatetimeToDatetime"]);
        Assert.Equal(new Value.Text(token.ToString("D")), stored["BinaryToGuid"]);
        Assert.Equal(new Value.Text(token.ToString("D")), stored["GuidToGuid"]);
    }
}
