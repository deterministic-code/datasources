using Datasources;
using SqliteCmd = Microsoft.Data.Sqlite.SqliteCommand;
using SqliteDb = Microsoft.Data.Sqlite.SqliteConnection;

namespace Datasources.Sqlite;

public sealed class SqliteDriver : ISqliteDriver
{
    SqliteDb? _db;

    public async Task Open(string filename)
    {
        if (_db is not null) return;
        _db = new SqliteDb(ConnectionString(filename));
        _db.DefaultTimeout = 0;
        await _db.OpenAsync();
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    public Task<SqliteExecuteResult> Execute(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? Task.FromResult(new SqliteExecuteResult(0, Value.Null.Instance)) : Run(sql, parameters);

    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult<IReadOnlyList<Row>>([])
            : Read(sql, parameters);

    public async Task Begin(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        await Exec(isolation == IsolationLevel.ReadUncommitted
            ? "PRAGMA read_uncommitted = 1"
            : "PRAGMA read_uncommitted = 0");
        await Exec("BEGIN");
    }
    public Task Commit() => Exec("COMMIT");
    public Task Rollback() => Exec("ROLLBACK");

    static string ConnectionString(string filename)
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

    SqliteDb RequireDb() => _db ?? throw new DatasourceException("SQLite driver is not open");

    async Task<SqliteExecuteResult> Run(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        var rows = await command.ExecuteNonQueryAsync();
        using var id = RequireDb().CreateCommand();
        id.CommandText = "SELECT last_insert_rowid()";
        return new SqliteExecuteResult(rows, Value.From(await id.ExecuteScalarAsync()));
    }

    async Task<IReadOnlyList<Row>> Read(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        using var reader = await command.ExecuteReaderAsync();
        var rows = new List<Row>();
        while (await reader.ReadAsync())
        {
            var row = new Row();
            for (var i = 0; i < reader.FieldCount; i++)
                row[reader.GetName(i)] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
            rows.Add(row);
        }
        return rows;
    }

    SqliteCmd Bind(string sql, Row parameters)
    {
        var command = RequireDb().CreateCommand();
        command.CommandText = sql;
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '$' or '@' or ':' ? name : $"${name}";
            command.Parameters.AddWithValue(key, value.ToObject() ?? DBNull.Value);
        }
        return command;
    }

    async Task Exec(string sql)
    {
        using var command = RequireDb().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
