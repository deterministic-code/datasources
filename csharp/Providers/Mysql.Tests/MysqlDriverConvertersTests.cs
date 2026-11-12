using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlDriverConvertersTests
{
    public static readonly Conversion[] TypeConverter =
    [
        new("string", "string", "TEXT"),
        new("character", "character", "CHAR(1)"),
        new("integer", "integer", "INT"),
        new("unsignedinteger", "unsignedinteger", "INT UNSIGNED"),
        new("smallinteger", "smallinteger", "SMALLINT"),
        new("unsignedsmallinteger", "unsignedsmallinteger", "SMALLINT UNSIGNED"),
        new("biginteger", "biginteger", "BIGINT"),
        new("unsignedbiginteger", "unsignedbiginteger", "BIGINT UNSIGNED"),
        new("float", "float", "DOUBLE"),
        new("decimal", "decimal", "DECIMAL(10, 2)"),
        new("boolean", "boolean", "TINYINT(1)"),
        new("datetime", "datetime", "DATETIME(3)"),
        new("binary", "binary", "BLOB"),
        new("uuid", "uuid", "CHAR(36)"),
        new("reference", "reference", "INT"),
        new("string", new SpecField { Type = "string" }, "TEXT"),
        new("string", new SpecField { Type = "string", Size = 64 }, "VARCHAR(64)"),
        new("string", new SpecField { Type = "string", Size = 20000 }, "LONGTEXT"),
        new("character", new SpecField { Type = "character", Size = 4 }, "CHAR(4)"),
        new("decimal", new SpecField { Type = "decimal", Size = new[] { 8, 3 } }, "DECIMAL(8, 3)"),
        new("binary", new SpecField { Type = "binary", Size = 16 }, "VARBINARY(16)"),
        new("binary", new SpecField { Type = "binary", Size = 70000 }, "LONGBLOB"),
        new("nope", "nope", typeof(InvalidOperationException)),
        new("decimal", new SpecField { Name = "amount", Type = "decimal", Size = 8 }, typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseTypeConverter =
    [
        new("integer", "INT", "integer"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] DefaultValueConverter =
    [
        new("datetime", "Now", "(NOW(3))"),
        new("datetime", "UtcNow", "(UTC_TIMESTAMP(3))"),
        new("uuid", "NewId", "(UUID())"),
        new("binary", "Hex('ab')", "(X'ab')"),
        new("boolean", "true", "1"),
        new("boolean", "false", "0"),
        new("string", "hi", "('hi')"),
        new("integer", "7", "7"),
        new("reference", "1", null),
        new("nope", "x", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseDefaultValueConverter =
    [
        new("binary", "(X'ab')", "Hex('ab')"),
        new("binary", "X'ab'", "Hex('ab')"),
        new("string", "('hi')", "String"),
        new("string", "'hi'", "String"),
        new("uuid", "(UUID())", "NewId"),
        new("binary", "X'ab')", typeof(InvalidOperationException)),
        new("string", "'hi')", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] RowTypeConverters =
    [
        new("boolean", 1L, true),
        new("boolean", 0L, false),
        new("datetime", "2020-01-02T03:04:05.000Z", new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc)),
        new("uuid", Guid.Parse("550e8400-e29b-41d4-a716-446655440000"), "550e8400-e29b-41d4-a716-446655440000"),
        new("uuid", Guid.Parse("550e8400-e29b-41d4-a716-446655440000").ToByteArray(), "550e8400-e29b-41d4-a716-446655440000"),
        new("decimal", 7L, "7"),
        new("decimal", 10.5d, "10.5"),
        new("decimal", "10.50", "10.5"),
        new("decimal", 0L, "0"),
        new("decimal", "0.00", "0"),
        new("binary", "AQIDBA==", new byte[] { 1, 2, 3, 4 }),
        new("integer", "10", 10L),
        new("integer", "0", 0L),
        new("integer", "1.0", typeof(FormatException)),
        new("integer", "", typeof(FormatException)),
    ];

    public static readonly Conversion[] ReverseRowTypeConverters =
    [
        new("boolean", true, 1L),
        new("boolean", false, 0L),
        new("datetime", "2020-01-02T03:04:05.000Z", "2020-01-02 03:04:05.000"),
        new("uuid", Guid.Parse("550e8400-e29b-41d4-a716-446655440000"), "550e8400-e29b-41d4-a716-446655440000"),
        new("decimal", 10.5m, "10.5"),
        new("binary", "AQIDBA==", new byte[] { 1, 2, 3, 4 }),
    ];

    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<MysqlDriver.TypeMap>(new MysqlDriver().TypeConverter);
        Conversion.Types(TypeConverter, types.Convert, types.Convert);
        Conversion.ReverseTypes(types, ReverseTypeConverter);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new MysqlDriver().DefaultValueConverter;
        Conversion.Defaults(defaults, DefaultValueConverter);
        Conversion.Defaults(defaults, ReverseDefaultValueConverter, reverse: true);
        Assert.Contains("00000000", defaults.Convert("uuid", "Empty"));
    }

    [Fact]
    public void ConvertsCells()
    {
        var converters = new MysqlDriver().RowTypeConverters;
        Conversion.Cells(converters, RowTypeConverters);
        Conversion.Cells(converters, ReverseRowTypeConverters, reverse: true);
    }
}
