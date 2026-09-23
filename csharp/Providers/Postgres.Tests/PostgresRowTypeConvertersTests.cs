using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresRowTypeConvertersTests
{
    [Fact]
    public void RegistersPostgresRowTypeConverters()
    {
        var types = new PostgresProvider().RowTypeConverters;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
