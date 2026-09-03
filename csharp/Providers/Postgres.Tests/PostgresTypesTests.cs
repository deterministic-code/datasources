using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresTypesTests
{
    [Fact]
    public void RegistersPostgresTypes()
    {
        _ = PostgresTypes.All;
        var types = DefaultTypeConverters.For("postgres");
        Assert.Equal(["bool", "int", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
