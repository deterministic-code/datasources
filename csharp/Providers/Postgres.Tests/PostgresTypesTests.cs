using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresTypesTests
{
    [Fact]
    public void RegistersPostgresTypes()
    {
        var types = new PostgresProvider().Types;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
