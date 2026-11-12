using Datasources;
using Datasources.SqlServer;

namespace Datasources.SqlServer.Tests;

public class SqlServerDriverConvertersTests
{
    [Fact]
    public void TypeConverter()
    {
        var driver = new SqlServerDriver();
        var types = Assert.IsType<TypeMap>(driver.TypeConverter);
        Assert.Equal("NVARCHAR(MAX)", types.Convert("string"));
        Assert.Equal("NCHAR(1)", types.Convert("character"));
        Assert.Equal("INT", types.Convert("integer"));
        Assert.Equal("BIGINT", types.Convert("unsignedinteger"));
        Assert.Equal("SMALLINT", types.Convert("smallinteger"));
        Assert.Equal("INT", types.Convert("unsignedsmallinteger"));
        Assert.Equal("BIGINT", types.Convert("biginteger"));
        Assert.Equal("DECIMAL(20)", types.Convert("unsignedbiginteger"));
        Assert.Equal("FLOAT", types.Convert("float"));
        Assert.Equal("DECIMAL", types.Convert("decimal"));
        Assert.Equal("BIT", types.Convert("boolean"));
        Assert.Equal("DATETIME2", types.Convert("datetime"));
        Assert.Equal("VARBINARY(MAX)", types.Convert("binary"));
        Assert.Equal("UNIQUEIDENTIFIER", types.Convert("uuid"));
        Assert.Equal("INT", types.Convert("reference"));
        Assert.Equal("NVARCHAR(64)", types.Convert(new SpecField { Type = "string", Size = 64 }));
        Assert.Equal("NCHAR(4)", types.Convert(new SpecField { Type = "character", Size = 4 }));
        Assert.Equal("DECIMAL(10, 2)", types.Convert(new SpecField { Type = "decimal", Size = new[] { 10, 2 } }));
        Assert.Equal("VARBINARY(16)", types.Convert(new SpecField { Type = "binary", Size = 16 }));
        Assert.Equal("integer", types.ReverseConvert("INT"));
        Assert.Equal("unsignedinteger", types.ReverseConvert("BIGINT"));
        Assert.Throws<InvalidOperationException>(() => types.Convert("nope"));
        Assert.Throws<InvalidOperationException>(() => types.ReverseConvert("nope"));
    }

    [Fact]
    public void DefaultValueConverter()
    {
        var defaults = new SqlServerDriver().DefaultValueConverter;
        Assert.Equal("GETDATE()", defaults.Convert("datetime", "Now"));
        Assert.Equal("GETUTCDATE()", defaults.Convert("datetime", "UtcNow"));
        Assert.Equal("NEWID()", defaults.Convert("uuid", "NewId"));
        Assert.Equal("'00000000-0000-0000-0000-000000000000'", defaults.Convert("uuid", "Empty"));
        Assert.Equal("0xab", defaults.Convert("binary", "Hex('ab')"));
        Assert.Equal("1", defaults.Convert("boolean", "true"));
        Assert.Equal("0", defaults.Convert("boolean", "false"));
        Assert.Equal("'hi'", defaults.Convert("string", "hi"));
        Assert.Equal("7", defaults.Convert("integer", "7"));
        Assert.Null(defaults.Convert("reference", "1"));
        Assert.Equal("Now", defaults.ReverseConvert("datetime", "GETDATE()"));
        Assert.Equal("UtcNow", defaults.ReverseConvert("datetime", "GETUTCDATE()"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", "NEWID()"));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "0xab"));
        Assert.Equal("String", defaults.ReverseConvert("string", "'hi'"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("nope", "x"));
    }

    [Fact]
    public void EmptyRowTypeConverters() =>
        Assert.Empty(new SqlServerDriver().RowTypeConverters);
}
