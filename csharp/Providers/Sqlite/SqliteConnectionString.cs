namespace Datasources.Sqlite;

internal static class SqliteConnectionString
{
    public static string ToConnectionString(string filename)
    {
        if (!filename.StartsWith("file:", StringComparison.OrdinalIgnoreCase)
            && !filename.Contains("mode=memory", StringComparison.OrdinalIgnoreCase)
            && !filename.Contains("cache=shared", StringComparison.OrdinalIgnoreCase))
            return $"Data Source={filename}";
        var name = filename.StartsWith("file:", StringComparison.OrdinalIgnoreCase) ? filename[5..] : filename;
        var query = name.IndexOf('?');
        if (query >= 0) name = name[..query];
        return $"Data Source={name};Mode=Memory;Cache=Shared";
    }
}
