using Datasources;

namespace Datasources.Postgres;

public static class PostgresProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("postgres");

    static PostgresProvider()
    {
        Datasource.Use(Factory);
    }
}
