using Datasources;
using Datasources.SqlServer;

namespace Datasources.SqlServer.Tests;

public class SqlServerDriverConvertersTests
{
    public static readonly Conversion[] TypeConverter =
    [
        new("string", "string", "NVARCHAR(MAX)"),
        new("character", "character", "NCHAR(1)"),
        new("integer", "integer", "INT"),
        new("unsignedinteger", "unsignedinteger", "BIGINT"),
        new("smallinteger", "smallinteger", "SMALLINT"),
        new("unsignedsmallinteger", "unsignedsmallinteger", "INT"),
        new("biginteger", "biginteger", "BIGINT"),
        new("unsignedbiginteger", "unsignedbiginteger", "DECIMAL(20)"),
        new("float", "float", "FLOAT"),
        new("decimal", "decimal", "DECIMAL"),
        new("boolean", "boolean", "BIT"),
        new("datetime", "datetime", "DATETIME2"),
        new("binary", "binary", "VARBINARY(MAX)"),
        new("uuid", "uuid", "UNIQUEIDENTIFIER"),
        new("reference", "reference", "INT"),
        new("string", new SpecField { Type = "string", Size = 64 }, "NVARCHAR(64)"),
        new("character", new SpecField { Type = "character", Size = 4 }, "NCHAR(4)"),
        new("decimal", new SpecField { Type = "decimal", Size = new[] { 10, 2 } }, "DECIMAL(10, 2)"),
        new("binary", new SpecField { Type = "binary", Size = 16 }, "VARBINARY(16)"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseTypeConverter =
    [
        new("integer", "INT", "integer"),
        new("unsignedinteger", "BIGINT", "unsignedinteger"),
        new("nope", "nope", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] DefaultValueConverter =
    [
        new("datetime", "Now", "GETDATE()"),
        new("datetime", "UtcNow", "GETUTCDATE()"),
        new("uuid", "NewId", "NEWID()"),
        new("uuid", "Empty", "'00000000-0000-0000-0000-000000000000'"),
        new("binary", "Hex('ab')", "0xab"),
        new("boolean", "true", "1"),
        new("boolean", "false", "0"),
        new("string", "hi", "'hi'"),
        new("integer", "7", "7"),
        new("reference", "1", null),
        new("nope", "x", typeof(InvalidOperationException)),
    ];

    public static readonly Conversion[] ReverseDefaultValueConverter =
    [
        new("datetime", "GETDATE()", "Now"),
        new("datetime", "GETUTCDATE()", "UtcNow"),
        new("uuid", "NEWID()", "NewId"),
        new("binary", "0xab", "Hex('ab')"),
        new("string", "'hi'", "String"),
    ];

    [Fact]
    public void ConvertsTypes()
    {
        var types = Assert.IsType<TypeMap>(new SqlServerDriver().TypeConverter);
        Conversion.Types(TypeConverter, types.Convert, types.Convert);
        Conversion.ReverseTypes(types, ReverseTypeConverter);
    }

    [Fact]
    public void ConvertsDefaults()
    {
        var defaults = new SqlServerDriver().DefaultValueConverter;
        Conversion.Defaults(defaults, DefaultValueConverter);
        Conversion.Defaults(defaults, ReverseDefaultValueConverter, reverse: true);
    }

    [Fact]
    public void EmptyRowTypeConverters() =>
        Assert.Empty(new SqlServerDriver().RowTypeConverters);
}
