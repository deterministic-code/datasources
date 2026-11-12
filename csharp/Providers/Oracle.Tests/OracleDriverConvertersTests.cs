using Datasources;
using Datasources.Oracle;

namespace Datasources.Oracle.Tests;

public class OracleDriverConvertersTests
{
    [Fact]
    public void TypeConverter()
    {
        var driver = new OracleDriver();
        var types = Assert.IsType<TypeMap>(driver.TypeConverter);
        Assert.Equal("CLOB", types.Convert("string"));
        Assert.Equal("CHAR(1)", types.Convert("character"));
        Assert.Equal("NUMBER(10)", types.Convert("integer"));
        Assert.Equal("NUMBER(10)", types.Convert("unsignedinteger"));
        Assert.Equal("NUMBER(5)", types.Convert("smallinteger"));
        Assert.Equal("NUMBER(5)", types.Convert("unsignedsmallinteger"));
        Assert.Equal("NUMBER(19)", types.Convert("biginteger"));
        Assert.Equal("NUMBER(20)", types.Convert("unsignedbiginteger"));
        Assert.Equal("BINARY_DOUBLE", types.Convert("float"));
        Assert.Equal("NUMBER", types.Convert("decimal"));
        Assert.Equal("NUMBER(1)", types.Convert("boolean"));
        Assert.Equal("TIMESTAMP", types.Convert("datetime"));
        Assert.Equal("BLOB", types.Convert("binary"));
        Assert.Equal("VARCHAR2(36)", types.Convert("uuid"));
        Assert.Equal("NUMBER(10)", types.Convert("reference"));
        Assert.Equal("VARCHAR2(64)", types.Convert(new SpecField { Type = "string", Size = 64 }));
        Assert.Equal("CHAR(4)", types.Convert(new SpecField { Type = "character", Size = 4 }));
        Assert.Equal("NUMBER(10, 2)", types.Convert(new SpecField { Type = "decimal", Size = new[] { 10, 2 } }));
        Assert.Equal("RAW(16)", types.Convert(new SpecField { Type = "binary", Size = 16 }));
        Assert.Equal("integer", types.ReverseConvert("NUMBER(10)"));
        Assert.Throws<InvalidOperationException>(() => types.Convert("nope"));
        Assert.Throws<InvalidOperationException>(() => types.ReverseConvert("nope"));
    }

    [Fact]
    public void DefaultValueConverter()
    {
        var defaults = new OracleDriver().DefaultValueConverter;
        Assert.Equal("LOCALTIMESTAMP", defaults.Convert("datetime", "Now"));
        Assert.Equal("SYS_EXTRACT_UTC(SYSTIMESTAMP)", defaults.Convert("datetime", "UtcNow"));
        Assert.Equal("SYS_GUID()", defaults.Convert("uuid", "NewId"));
        Assert.Equal("'00000000-0000-0000-0000-000000000000'", defaults.Convert("uuid", "Empty"));
        Assert.Equal("HEXTORAW('ab')", defaults.Convert("binary", "Hex('ab')"));
        Assert.Equal("1", defaults.Convert("boolean", "true"));
        Assert.Equal("0", defaults.Convert("boolean", "false"));
        Assert.Equal("'hi'", defaults.Convert("string", "hi"));
        Assert.Equal("7", defaults.Convert("integer", "7"));
        Assert.Null(defaults.Convert("reference", "1"));
        Assert.Equal("Now", defaults.ReverseConvert("datetime", "LOCALTIMESTAMP"));
        Assert.Equal("UtcNow", defaults.ReverseConvert("datetime", "SYS_EXTRACT_UTC(SYSTIMESTAMP)"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", "SYS_GUID()"));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "HEXTORAW('ab')"));
        Assert.Equal("String", defaults.ReverseConvert("string", "'hi'"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("nope", "x"));
    }

    [Fact]
    public void EmptyRowTypeConverters() =>
        Assert.Empty(new OracleDriver().RowTypeConverters);
}
