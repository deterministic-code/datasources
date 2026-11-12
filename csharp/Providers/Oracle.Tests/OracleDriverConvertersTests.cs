using Datasources;
using Datasources.Oracle;

namespace Datasources.Oracle.Tests;

public class OracleDriverConvertersTests
{
    public static readonly Conversion[] TypeConverter =
    [
        new("string", "string", "CLOB"),
        new("character", "character", "CHAR(1)"),
        new("integer", "integer", "NUMBER(10)"),
        new("unsignedinteger", "unsignedinteger", "NUMBER(10)"),
        new("smallinteger", "smallinteger", "NUMBER(5)"),
        new("unsignedsmallinteger", "unsignedsmallinteger", "NUMBER(5)"),
        new("biginteger", "biginteger", "NUMBER(19)"),
        new("unsignedbiginteger", "unsignedbiginteger", "NUMBER(20)"),
        new("float", "float", "BINARY_DOUBLE"),
        new("decimal", "decimal", "NUMBER"),
        new("boolean", "boolean", "NUMBER(1)"),
        new("datetime", "datetime", "TIMESTAMP"),
        new("binary", "binary", "BLOB"),
        new("uuid", "uuid", "VARCHAR2(36)"),
        new("reference", "reference", "NUMBER(10)"),
        new("string", new SpecField { Type = "string", Size = 64 }, "VARCHAR2(64)"),
        new("character", new SpecField { Type = "character", Size = 4 }, "CHAR(4)"),
        new("decimal", new SpecField { Type = "decimal", Size = new[] { 10, 2 } }, "NUMBER(10, 2)"),
        new("binary", new SpecField { Type = "binary", Size = 16 }, "RAW(16)"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseTypeConverter =
    [
        new("integer", "NUMBER(10)", "integer"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] DefaultValueConverter =
    [
        new("datetime", "Now", "LOCALTIMESTAMP"),
        new("datetime", "UtcNow", "SYS_EXTRACT_UTC(SYSTIMESTAMP)"),
        new("uuid", "NewId", "SYS_GUID()"),
        new("uuid", "Empty", "'00000000-0000-0000-0000-000000000000'"),
        new("binary", "Hex('ab')", "HEXTORAW('ab')"),
        new("boolean", "true", "1"),
        new("boolean", "false", "0"),
        new("string", "hi", "'hi'"),
        new("integer", "7", "7"),
        new("reference", "1", null),
        new("nope", "x", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseDefaultValueConverter =
    [
        new("datetime", "LOCALTIMESTAMP", "Now"),
        new("datetime", "SYS_EXTRACT_UTC(SYSTIMESTAMP)", "UtcNow"),
        new("uuid", "SYS_GUID()", "NewId"),
        new("binary", "HEXTORAW('ab')", "Hex('ab')"),
        new("string", "'hi'", "String"),
    ];

    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new OracleDriver().TypeConverter);
        Conversion.Types(TypeConverter, types.Convert, types.Convert);
        Conversion.ReverseTypes(types, ReverseTypeConverter);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new OracleDriver().DefaultValueConverter;
        Conversion.Defaults(defaults, DefaultValueConverter);
        Conversion.Defaults(defaults, ReverseDefaultValueConverter, reverse: true);
    }

    [Fact]
    public void EmptyRowTypeConverters() =>
        Assert.Empty(new OracleDriver().RowTypeConverters);
}
