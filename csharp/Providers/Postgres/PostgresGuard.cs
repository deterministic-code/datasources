namespace Datasources.Postgres;

public static class PostgresGuard
{
    public static IPostgresDriver Require(IPostgresDriver? driver) =>
        driver ?? throw new DatasourceException("Postgres driver is required");
}
