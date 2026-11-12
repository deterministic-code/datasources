using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresDriverConvertersTests
{
    public static readonly Conversion[] TypeConverter =
    [
        new("string", "string", "TEXT"),
        new("character", "character", "CHAR(1)"),
        new("integer", "integer", "INTEGER"),
        new("unsignedinteger", "unsignedinteger", "BIGINT"),
        new("smallinteger", "smallinteger", "SMALLINT"),
        new("unsignedsmallinteger", "unsignedsmallinteger", "INTEGER"),
        new("biginteger", "biginteger", "BIGINT"),
        new("unsignedbiginteger", "unsignedbiginteger", "NUMERIC(20)"),
        new("float", "float", "DOUBLE PRECISION"),
        new("decimal", "decimal", "NUMERIC"),
        new("boolean", "boolean", "BOOLEAN"),
        new("datetime", "datetime", "TIMESTAMPTZ"),
        new("binary", "binary", "BYTEA"),
        new("uuid", "uuid", "UUID"),
        new("reference", "reference", "INTEGER"),
        new("string", new SpecField { Type = "string", Size = 64 }, "VARCHAR(64)"),
        new("character", new SpecField { Type = "character", Size = 4 }, "CHAR(4)"),
        new("decimal", new SpecField { Type = "decimal", Size = new[] { 10, 2 } }, "NUMERIC(10, 2)"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseTypeConverter =
    [
        new("integer", "INTEGER", "integer"),
        new("unsignedinteger", "BIGINT", "unsignedinteger"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] DefaultValueConverter =
    [
        new("datetime", "Now", "LOCALTIMESTAMP"),
        new("datetime", "UtcNow", "(NOW() AT TIME ZONE 'UTC')"),
        new("uuid", "NewId", "gen_random_uuid()"),
        new("uuid", "Empty", "'00000000-0000-0000-0000-000000000000'"),
        new("binary", "Hex('ab')", "'\\xab'"),
        new("boolean", "true", "TRUE"),
        new("boolean", "false", "FALSE"),
        new("string", "hi", "'hi'"),
        new("integer", "7", "7"),
        new("reference", "1", null),
        new("nope", "x", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseDefaultValueConverter =
    [
        new("datetime", "LOCALTIMESTAMP", "Now"),
        new("datetime", "(NOW() AT TIME ZONE 'UTC')", "UtcNow"),
        new("uuid", "gen_random_uuid()", "NewId"),
        new("binary", "'\\xab'", "Hex('ab')"),
        new("string", "'hi'", "String"),
        new("datetime", "nope", typeof(InvalidOperationException)),
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
        new("uuid", Guid.Parse("550e8400-e29b-41d4-a716-446655440000"), "550e8400-e29b-41d4-a716-446655440000"),
        new("decimal", 10.5m, "10.5"),
        new("binary", "AQIDBA==", new byte[] { 1, 2, 3, 4 }),
    ];

    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new PostgresDriver().TypeConverter);
        Conversion.Types(TypeConverter, types.Convert, types.Convert);
        Conversion.ReverseTypes(types, ReverseTypeConverter);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new PostgresDriver().DefaultValueConverter;
        Conversion.Defaults(defaults, DefaultValueConverter);
        Conversion.Defaults(defaults, ReverseDefaultValueConverter, reverse: true);
    }

    [Fact]
    public void ConvertsCells()
    {
        var converters = new PostgresDriver().RowTypeConverters;
        Conversion.Cells(converters, RowTypeConverters);
        Conversion.Cells(converters, ReverseRowTypeConverters, reverse: true);
    }
}
