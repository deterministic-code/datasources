using Datasources;
using SqliteDb = Microsoft.Data.Sqlite.SqliteConnection;

namespace Datasources.Sqlite;

internal sealed class SqliteConnectionDriver : ISqliteConnectionSession
{
    SqliteDb? _db;

    public async Task Open(string filename)
    {
        if (_db is not null) return;
        _db = new SqliteDb(SqliteConnectionString.ToConnectionString(filename));
        _db.DefaultTimeout = 0;
        await _db.OpenAsync();
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    internal SqliteDb Require() =>
        _db ?? throw new DatasourceException("SQLite driver is not open");
}
