using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresRowTypeConvertersTests
{
    [Fact]
    public void RegistersPostgresRowTypeConverters()
    {
        var types = new PostgresDriver().RowTypeConverters;
        Assert.Equal(
            [
                "boolean", "datetime", "uuid", "decimal", "binary", "integer",
                "unsignedinteger", "smallinteger", "unsignedsmallinteger", "biginteger",
                "unsignedbiginteger",
            ],
            types.Keys);
    }
}
