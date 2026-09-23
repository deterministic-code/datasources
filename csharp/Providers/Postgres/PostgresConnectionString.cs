namespace Datasources.Postgres;

internal static class PostgresConnectionString
{
    public static string ToConnectionString(string url)
    {
        if (!url.StartsWith("postgres://", StringComparison.OrdinalIgnoreCase)
            && !url.StartsWith("postgresql://", StringComparison.OrdinalIgnoreCase))
            return url;
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 5432;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        var username = user.Length > 0 ? Uri.UnescapeDataString(user[0]) : "";
        return $"Host={uri.Host};Port={port};Username={username};Password={password};Database={database}";
    }
}
