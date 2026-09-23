namespace Datasources.Postgres;

internal sealed class PostgresConnectionOptions
{
    public string? Url { get; set; }
    public string? Schema { get; set; }
}
