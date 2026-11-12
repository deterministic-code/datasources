using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteDriverConvertersTests
{
    public static readonly Conversion[] TypeConverter =
    [
        new("string", "string", "TEXT"),
        new("character", "character", "TEXT"),
        new("integer", "integer", "INTEGER"),
        new("unsignedinteger", "unsignedinteger", "INTEGER"),
        new("smallinteger", "smallinteger", "INTEGER"),
        new("unsignedsmallinteger", "unsignedsmallinteger", "INTEGER"),
        new("biginteger", "biginteger", "INTEGER"),
        new("unsignedbiginteger", "unsignedbiginteger", "INTEGER"),
        new("float", "float", "REAL"),
        new("decimal", "decimal", "NUMERIC"),
        new("boolean", "boolean", "BOOLEAN"),
        new("datetime", "datetime", "TEXT"),
        new("binary", "binary", "BLOB"),
        new("uuid", "uuid", "TEXT"),
        new("reference", "reference", "INTEGER"),
        new("string", new SpecField { Type = "string" }, "TEXT"),
        new("string", new SpecField { Type = "string", Size = "unlimited" }, "TEXT"),
        new("string", new SpecField { Type = "string", Size = 64 }, "VARCHAR(64)"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseTypeConverter =
    [
        new("string", "TEXT", "string"),
        new("integer", "INTEGER", "integer"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] DefaultValueConverter =
    [
        new("datetime", "Now", "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))"),
        new("datetime", "UtcNow", "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))"),
        new("datetime", "DateTime('2020-01-01T00:00:00Z')", "'2020-01-01T00:00:00Z'"),
        new("uuid", "Empty", "'00000000-0000-0000-0000-000000000000'"),
        new("uuid", "uuid('11111111-1111-1111-1111-111111111111')", "'11111111-1111-1111-1111-111111111111'"),
        new("binary", "Hex('ab')", "X'ab'"),
        new("boolean", "true", "1"),
        new("boolean", "false", "0"),
        new("string", "hi", "'hi'"),
        new("integer", "7", "7"),
        new("reference", "1", null),
        new("nope", "x", typeof(InvalidOperationException)),
        new("datetime", "2020-01-01T00:00:00Z", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseDefaultValueConverter =
    [
        new("datetime", "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))", "Now"),
        new("datetime", "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))", "UtcNow"),
        new("datetime", "'2020-01-01T00:00:00Z'", "DateTime('2020-01-01T00:00:00Z')"),
        new("binary", "X'ab'", "Hex('ab')"),
        new("string", "'hi'", "String"),
        new("string", "oops", typeof(InvalidOperationException)),
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
        new("integer", "10", 10L),
        new("integer", "0", 0L),
        new("integer", "", typeof(FormatException)),
        new("integer", "1.0", typeof(FormatException)),
    ];

    public static readonly Conversion[] ReverseRowTypeConverters =
    [
        new("boolean", true, 1L),
        new("boolean", false, 0L),
        new("uuid", Guid.Parse("550e8400-e29b-41d4-a716-446655440000"), "550e8400-e29b-41d4-a716-446655440000"),
        new("decimal", 10.5m, "10.5"),
    ];

    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new SqliteDriver().TypeConverter);
        Conversion.Types(TypeConverter, types.Convert, types.Convert);
        Conversion.ReverseTypes(types, ReverseTypeConverter);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new SqliteDriver().DefaultValueConverter;
        Conversion.Defaults(defaults, DefaultValueConverter);
        Conversion.Defaults(defaults, ReverseDefaultValueConverter, reverse: true);
        Assert.Contains("randomblob", defaults.Convert("uuid", "NewId"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", defaults.Convert("uuid", "NewId")!));
    }

    [Fact]
    public void ConvertsCells()
    {
        var converters = new SqliteDriver().RowTypeConverters;
        Conversion.Cells(converters, RowTypeConverters);
        Conversion.Cells(converters, ReverseRowTypeConverters, reverse: true);
    }
}
