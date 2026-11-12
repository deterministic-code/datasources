using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteDriverConvertersTests
{
    [Fact]
    public void TypeConverter()
    {
        var driver = new SqliteDriver();
        var types = Assert.IsType<TypeMap>(driver.TypeConverter);
        Assert.Equal("TEXT", types.Convert("string"));
        Assert.Equal("TEXT", types.Convert("character"));
        Assert.Equal("INTEGER", types.Convert("integer"));
        Assert.Equal("INTEGER", types.Convert("unsignedinteger"));
        Assert.Equal("INTEGER", types.Convert("smallinteger"));
        Assert.Equal("INTEGER", types.Convert("unsignedsmallinteger"));
        Assert.Equal("INTEGER", types.Convert("biginteger"));
        Assert.Equal("INTEGER", types.Convert("unsignedbiginteger"));
        Assert.Equal("REAL", types.Convert("float"));
        Assert.Equal("NUMERIC", types.Convert("decimal"));
        Assert.Equal("BOOLEAN", types.Convert("boolean"));
        Assert.Equal("TEXT", types.Convert("datetime"));
        Assert.Equal("BLOB", types.Convert("binary"));
        Assert.Equal("TEXT", types.Convert("uuid"));
        Assert.Equal("INTEGER", types.Convert("reference"));
        Assert.Equal("TEXT", types.Convert(new SpecField { Type = "string" }));
        Assert.Equal("TEXT", types.Convert(new SpecField { Type = "string", Size = "unlimited" }));
        Assert.Equal("VARCHAR(64)", types.Convert(new SpecField { Type = "string", Size = 64 }));
        Assert.Equal("string", types.ReverseConvert("TEXT"));
        Assert.Equal("integer", types.ReverseConvert("INTEGER"));
        Assert.Throws<InvalidOperationException>(() => types.Convert("nope"));
        Assert.Throws<InvalidOperationException>(() => types.ReverseConvert("nope"));
    }

    [Fact]
    public void DefaultValueConverter()
    {
        var defaults = new SqliteDriver().DefaultValueConverter;
        Assert.Equal("(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))", defaults.Convert("datetime", "Now"));
        Assert.Equal("(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))", defaults.Convert("datetime", "UtcNow"));
        Assert.Equal("'2020-01-01T00:00:00Z'", defaults.Convert("datetime", "DateTime('2020-01-01T00:00:00Z')"));
        Assert.Contains("randomblob", defaults.Convert("uuid", "NewId"));
        Assert.Equal("'00000000-0000-0000-0000-000000000000'", defaults.Convert("uuid", "Empty"));
        Assert.Equal(
            "'11111111-1111-1111-1111-111111111111'",
            defaults.Convert("uuid", "uuid('11111111-1111-1111-1111-111111111111')"));
        Assert.Equal("X'ab'", defaults.Convert("binary", "Hex('ab')"));
        Assert.Equal("1", defaults.Convert("boolean", "true"));
        Assert.Equal("0", defaults.Convert("boolean", "false"));
        Assert.Equal("'hi'", defaults.Convert("string", "hi"));
        Assert.Equal("7", defaults.Convert("integer", "7"));
        Assert.Null(defaults.Convert("reference", "1"));
        Assert.Equal("Now", defaults.ReverseConvert("datetime", "(strftime('%Y-%m-%dT%H:%M:%f', 'now', 'localtime'))"));
        Assert.Equal("UtcNow", defaults.ReverseConvert("datetime", "(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))"));
        Assert.Equal("DateTime('2020-01-01T00:00:00Z')", defaults.ReverseConvert("datetime", "'2020-01-01T00:00:00Z'"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", defaults.Convert("uuid", "NewId")!));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "X'ab'"));
        Assert.Equal("String", defaults.ReverseConvert("string", "'hi'"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("nope", "x"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("datetime", "2020-01-01T00:00:00Z"));
        Assert.Throws<InvalidOperationException>(() => defaults.ReverseConvert("string", "oops"));
    }
}
