using Datasources;
using SqliteDb = Microsoft.Data.Sqlite.SqliteConnection;

namespace Datasources.Sqlite;

internal sealed class SqliteConnectionDriver : ISqliteConnectionSession
{
    SqliteDb? _db;

    public async Task Open(string filename, SqliteOptions options)
    {
        if (_db is not null) return;
        if (options.Error is not null)
            throw new DatasourceException(options.Error);
        _db = new SqliteDb(SqliteConnectionString.ToConnectionString(filename));
        _db.DefaultTimeout = options.ResolvedBusyTimeout();
        await _db.OpenAsync();
        var journal = options.ResolvedJournalMode(filename);
        if (journal is not null && !SqliteOptions.IsMemory(filename))
            await Pragma($"journal_mode = {journal}");
        await Pragma($"busy_timeout = {options.ResolvedBusyTimeout()}");
        if (options.Synchronous is not null)
            await Pragma($"synchronous = {options.Synchronous}");
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    internal SqliteDb Require() =>
        _db ?? throw new DatasourceException("SQLite driver is not open");

    async Task Pragma(string body)
    {
        await using var command = Require().CreateCommand();
        command.CommandText = $"PRAGMA {body}";
        await command.ExecuteNonQueryAsync();
    }
}
