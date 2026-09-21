namespace Datasources.Postgres;

public static class PostgresQuote
{
    public static string Quote(string id) => $"\"{id.Replace("\"", "\"\"")}\"";
    public static string Qualified(string id) => string.Join(".", id.Split('.').Select(Quote));
}
