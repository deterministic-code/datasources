namespace Datasources.Postgres;

internal static class PostgresGuard
{
    public static IPostgresDriver Require(IPostgresDriver? driver) =>
        driver ?? throw new DatasourceException("Postgres driver is required");
}
