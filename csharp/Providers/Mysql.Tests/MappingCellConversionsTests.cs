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
}
