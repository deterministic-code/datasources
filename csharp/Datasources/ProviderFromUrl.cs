namespace Datasources;

public static class ProviderFromUrl
{
    public static string? From(string? url)
    {
        if (string.IsNullOrWhiteSpace(url)) return null;
        var colon = url.IndexOf(':');
        if (colon <= 0) return null;
        var scheme = url[..colon].ToLowerInvariant();
        return scheme switch
        {
            "postgresql" => "postgres",
            "mssql" => "sqlserver",
            "sqlite3" => "sqlite",
            _ => scheme
        };
    }
}
