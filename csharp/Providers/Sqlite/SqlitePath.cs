namespace Datasources.Sqlite;

internal static class SqlitePath
{
    public static string? FromUrl(string? url)
    {
        if (url is null || !url.StartsWith("sqlite:", StringComparison.OrdinalIgnoreCase))
            return null;
        var rest = url[7..];
        var path = rest.StartsWith("///") ? rest[3..] : rest.StartsWith("//") ? rest[2..] : rest;
        return path is "" or ":memory" or ":memory:" ? ":memory:" : path;
    }
}
