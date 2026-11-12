using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresDriverConvertersTests
{
    [Fact]
    public void TypeConverter()
    {
        var driver = new PostgresDriver();
        var types = Assert.IsType<TypeMap>(driver.TypeConverter);
        Assert.Equal("TEXT", types.Convert("string"));
        Assert.Equal("CHAR(1)", types.Convert("character"));
        Assert.Equal("INTEGER", types.Convert("integer"));
        Assert.Equal("BIGINT", types.Convert("unsignedinteger"));
        Assert.Equal("SMALLINT", types.Convert("smallinteger"));
        Assert.Equal("INTEGER", types.Convert("unsignedsmallinteger"));
        Assert.Equal("BIGINT", types.Convert("biginteger"));
        Assert.Equal("NUMERIC(20)", types.Convert("unsignedbiginteger"));
        Assert.Equal("DOUBLE PRECISION", types.Convert("float"));
        Assert.Equal("NUMERIC", types.Convert("decimal"));
        Assert.Equal("BOOLEAN", types.Convert("boolean"));
        Assert.Equal("TIMESTAMPTZ", types.Convert("datetime"));
        Assert.Equal("BYTEA", types.Convert("binary"));
        Assert.Equal("UUID", types.Convert("uuid"));
        Assert.Equal("INTEGER", types.Convert("reference"));
        Assert.Equal("VARCHAR(64)", types.Convert(new SpecField { Type = "string", Size = 64 }));
        Assert.Equal("CHAR(4)", types.Convert(new SpecField { Type = "character", Size = 4 }));
        Assert.Equal("NUMERIC(10, 2)", types.Convert(new SpecField { Type = "decimal", Size = new[] { 10, 2 } }));
        Assert.Equal("integer", types.ReverseConvert("INTEGER"));
        Assert.Equal("unsignedinteger", types.ReverseConvert("BIGINT"));
        Assert.Throws<InvalidOperationException>(() => types.Convert("nope"));
        Assert.Throws<InvalidOperationException>(() => types.ReverseConvert("nope"));
    }

    [Fact]
    public void DefaultValueConverter()
    {
        var defaults = new PostgresDriver().DefaultValueConverter;
        Assert.Equal("LOCALTIMESTAMP", defaults.Convert("datetime", "Now"));
        Assert.Equal("(NOW() AT TIME ZONE 'UTC')", defaults.Convert("datetime", "UtcNow"));
        Assert.Equal("gen_random_uuid()", defaults.Convert("uuid", "NewId"));
        Assert.Equal("'00000000-0000-0000-0000-000000000000'", defaults.Convert("uuid", "Empty"));
        Assert.Equal("'\\xab'", defaults.Convert("binary", "Hex('ab')"));
        Assert.Equal("TRUE", defaults.Convert("boolean", "true"));
        Assert.Equal("FALSE", defaults.Convert("boolean", "false"));
        Assert.Equal("'hi'", defaults.Convert("string", "hi"));
        Assert.Equal("7", defaults.Convert("integer", "7"));
        Assert.Null(defaults.Convert("reference", "1"));
        Assert.Equal("Now", defaults.ReverseConvert("datetime", "LOCALTIMESTAMP"));
        Assert.Equal("UtcNow", defaults.ReverseConvert("datetime", "(NOW() AT TIME ZONE 'UTC')"));
        Assert.Equal("NewId", defaults.ReverseConvert("uuid", "gen_random_uuid()"));
        Assert.Equal("Hex('ab')", defaults.ReverseConvert("binary", "'\\xab'"));
        Assert.Equal("String", defaults.ReverseConvert("string", "'hi'"));
        Assert.Throws<InvalidOperationException>(() => defaults.Convert("nope", "x"));
        Assert.Throws<InvalidOperationException>(() => defaults.ReverseConvert("datetime", "nope"));
    }
}
