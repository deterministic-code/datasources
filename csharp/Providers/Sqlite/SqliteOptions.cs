namespace Datasources.Sqlite;

internal sealed class SqliteOptions
{
    public string? JournalMode { get; init; }
    public int? BusyTimeout { get; init; }
    public int? PoolSize { get; init; }
    public string? Synchronous { get; init; }
    public string? Error { get; init; }

    public static SqliteOptions Empty { get; } = new();

    public static bool IsMemory(string filename) =>
        filename is ":memory:" or ":memory"
        || filename.Contains("mode=memory", StringComparison.OrdinalIgnoreCase);

    public string? ResolvedJournalMode(string filename) =>
        IsMemory(filename) ? null : JournalMode ?? "WAL";

    public int ResolvedBusyTimeout() => BusyTimeout ?? 5000;

    public int ResolvedPoolSize(string filename) =>
        IsMemory(filename)
            ? 0
            : PoolSize ?? (ResolvedJournalMode(filename) == "WAL" ? 8 : 0);
}
