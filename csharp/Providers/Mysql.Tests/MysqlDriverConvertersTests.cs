using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlDriverConvertersTests
{
    [Fact]
    public void TypeConverter()
    {
        var driver = new MysqlDriver();
        var types = Assert.IsType<MysqlDriver.TypeMap>(driver.TypeConverter);
        Assert.Equal("TEXT", types.Convert("string"));
        Assert.Equal("CHAR(1)", types.Convert("character"));
        Assert.Equal("INT", types.Convert("integer"));
        Assert.Equal("INT UNSIGNED", types.Convert("unsignedinteger"));
        Assert.Equal("SMALLINT", types.Convert("smallinteger"));
        Assert.Equal("SMALLINT UNSIGNED", types.Convert("unsignedsmallinteger"));
        Assert.Equal("BIGINT", types.Convert("biginteger"));
        Assert.Equal("BIGINT UNSIGNED", types.Convert("unsignedbiginteger"));
        Assert.Equal("DOUBLE", types.Convert("float"));
        Assert.Equal("DECIMAL(10, 2)", types.Convert("decimal"));
        Assert.Equal("TINYINT(1)", types.Convert("boolean"));
        Assert.Equal("DATETIME(3)", types.Convert("datetime"));
        Assert.Equal("BLOB", types.Convert("binary"));
        Assert.Equal("CHAR(36)", types.Convert("uuid"));
        Assert.Equal("INT", types.Convert("reference"));
        Assert.Equal("TEXT", types.Convert(new SpecField { Type = "string" }));
        Assert.Equal("VARCHAR(64)", types.Convert(new SpecField { Type = "string", Size = 64 }));
        Assert.Equal("LONGTEXT", types.Convert(new SpecField { Type = "string", Size = 20000 }));
        Assert.Equal("CHAR(4)", types.Convert(new SpecField { Type = "character", Size = 4 }));
        Assert.Equal("DECIMAL(8, 3)", types.Convert(new SpecField { Type = "decimal", Size = new[] { 8, 3 } }));
        Assert.Equal("VARBINARY(16)", types.Convert(new SpecField { Type = "binary", Size = 16 }));
        Assert.Equal("LONGBLOB", types.Convert(new SpecField { Type = "binary", Size = 70000 }));
        Assert.Equal("integer", types.ReverseConvert("INT"));
        Assert.Throws<InvalidOperationException>(() => types.Convert("nope"));
        Assert.Throws<InvalidOperationException>(() => types.ReverseConvert("nope"));
        Assert.Throws<InvalidOperationException>(() =>
            types.Convert(new SpecField { Name = "amount", Type = "decimal", Size = 8 }));
    }

    [Fact]
    public void DefaultValueConverter()
    {
        var defaults = new MysqlDriver().DefaultValueConverter;
        Assert.Equal("(NOW(3))", defaults.Convert("datetime", "Now"));
        Assert.Equal("(UTC_TIMESTAMP(3))", defaults.Convert("datetime", "UtcNow"));
        Assert.Equal("(UUID())", defaults.Convert("uuid", "NewId"));
        Assert.Contains("00000000", defaults.Convert("uuid", "Empty"));
        Assert.Equal("(X'ab')", defaults.Convert("binary", "Hex('ab')"));
        Assert.Equal("1", defaults.Convert("boolean", "true"));
        Assert.Equal("0", defaults.Convert("boolean", "false"));
        Assert.Equal("('hi')", defaults.Convert("string", "hi"));
        Assert.Equal("7", defaults.Convert("integer", "7"));
        Assert.Null(defaults.Convert("reference", "1"));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "(X'ab')"));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "X'ab'"));
        Assert.Equal("String", defaults.ReverseConvert("string", "('hi')"));
        Assert.Equal("String", defaults.ReverseConvert("string", "'hi'"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", "(UUID())"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("nope", "x"));
        Assert.Throws<InvalidOperationException>(() => defaults.ReverseConvert("binary", "X'ab')"));
        Assert.Throws<InvalidOperationException>(() => defaults.ReverseConvert("string", "'hi')"));
    }
}
