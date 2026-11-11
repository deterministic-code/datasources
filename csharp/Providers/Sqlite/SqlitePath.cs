namespace Datasources.Sqlite;

internal readonly record struct SqlitePathResult(string Filename, SqliteOptions Options);

internal static class SqlitePath
{
    static readonly string[] Known = ["journal_mode", "busy_timeout", "pool_size", "synchronous"];

    public static string? FromUrl(string? url) => Parse(url)?.Filename;

    public static SqlitePathResult? Parse(string? url)
    {
        if (url is null || !url.StartsWith("sqlite:", StringComparison.OrdinalIgnoreCase))
            return null;
        var rest = url[7..];
        var path = rest.StartsWith("///") ? rest[3..] : rest.StartsWith("//") ? rest[2..] : rest;
        path = path is "" or ":memory" or ":memory:" ? ":memory:" : path;
        return ParseFilenameQuery(path);
    }

    public static SqlitePathResult ParseFilenameQuery(string path)
    {
        var queryAt = path.IndexOf('?');
        if (queryAt < 0)
            return new SqlitePathResult(path, SqliteOptions.Empty);
        var file = path[..queryAt];
        var kept = new List<string>();
        string? journalMode = null;
        int? busyTimeout = null;
        int? poolSize = null;
        string? synchronous = null;
        string? error = null;
        foreach (var pair in path[(queryAt + 1)..].Split('&'))
        {
            if (pair.Length == 0)
                continue;
            var eq = pair.IndexOf('=');
            var key = eq < 0 ? pair : pair[..eq];
            var value = eq < 0 ? "" : pair[(eq + 1)..];
            if (!Known.Any(name => name.Equals(key, StringComparison.OrdinalIgnoreCase)))
            {
                kept.Add(pair);
                continue;
            }
            error ??= Assign(key, value, ref journalMode, ref busyTimeout, ref poolSize, ref synchronous);
        }
        var filename = kept.Count == 0 ? file : $"{file}?{string.Join("&", kept)}";
        return new SqlitePathResult(
            filename,
            new SqliteOptions
            {
                JournalMode = journalMode,
                BusyTimeout = busyTimeout,
                PoolSize = poolSize,
                Synchronous = synchronous,
                Error = error,
            });
    }

    static string? Assign(
        string key,
        string value,
        ref string? journalMode,
        ref int? busyTimeout,
        ref int? poolSize,
        ref string? synchronous) =>
        key.ToLowerInvariant() switch
        {
            "journal_mode" => AssignJournal(value, ref journalMode),
            "busy_timeout" => AssignInt(value, 0, int.MaxValue, "busy_timeout", ref busyTimeout),
            "pool_size" => AssignInt(value, 0, 64, "pool_size", ref poolSize),
            "synchronous" => AssignSynchronous(value, ref synchronous),
            _ => $"Invalid sqlite option {key}={value}",
        };

    static string? AssignJournal(string value, ref string? journalMode)
    {
        var mode = value.ToLowerInvariant() switch
        {
            "wal" => "WAL",
            "delete" => "DELETE",
            "truncate" => "TRUNCATE",
            "persist" => "PERSIST",
            "memory" => "MEMORY",
            "off" => "OFF",
            _ => null,
        };
        if (mode is null)
            return $"Invalid sqlite option journal_mode={value}";
        journalMode = mode;
        return null;
    }

    static string? AssignSynchronous(string value, ref string? synchronous)
    {
        var mode = value.ToLowerInvariant() switch
        {
            "off" => "OFF",
            "normal" => "NORMAL",
            "full" => "FULL",
            "extra" => "EXTRA",
            _ => null,
        };
        if (mode is null)
            return $"Invalid sqlite option synchronous={value}";
        synchronous = mode;
        return null;
    }

    static string? AssignInt(string value, int min, int max, string name, ref int? target)
    {
        if (!int.TryParse(value, out var parsed) || parsed < min || parsed > max)
            return $"Invalid sqlite option {name}={value}";
        target = parsed;
        return null;
    }
}
