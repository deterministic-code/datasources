using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteRowTypeConvertersTests
{
    [Fact]
    public void RegistersSqliteRowTypeConverters()
    {
        var types = new SqliteDriver().RowTypeConverters;
        Assert.Equal(
            [
                "boolean", "datetime", "uuid", "decimal", "binary", "integer",
                "unsignedinteger", "smallinteger", "unsignedsmallinteger", "biginteger", "unsignedbiginteger",
            ],
            types.Keys);
    }

    [Fact]
    public void ConvertsBigintTextFloorThroughUnsignedInteger()
    {
        var schema = new Schema(new Dictionary<string, SchemaColumn>
        {
            ["Floor"] = new("unsignedinteger", true, false),
        });
        var integer = new SqliteDriver().RowTypeConverters["integer"];
        var missed = RowTypeConverter.From(
            new Dictionary<string, TypeConverter<Value, Value>> { ["integer"] = integer },
            schema.Types);
        Assert.Equal("2", missed.Convert(new Row { ["Floor"] = "2" })["Floor"]);
        var converted = RowTypeConverter.From(new SqliteDriver().RowTypeConverters, schema.Types);
        Assert.Equal(2L, converted.Convert(new Row { ["Floor"] = "2" })["Floor"]);
    }
}
