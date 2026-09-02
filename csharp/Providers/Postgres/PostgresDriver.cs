using Npgsql;

namespace Datasources.Postgres;

public sealed class PostgresDriver : IPostgresDriver
{
    readonly string? _schema;
    NpgsqlConnection? _db;

    public PostgresDriver(string? schema = null) => _schema = schema;

    public async Task Open(string url)
    {
        if (_db is not null) return;
        _db = new NpgsqlConnection(ToConnectionString(url));
        await _db.OpenAsync();
        if (!string.IsNullOrEmpty(_schema))
            await Exec($"SET search_path TO {_schema}");
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    public Task<PostgresExecuteResult> Execute(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? Task.FromResult(new PostgresExecuteResult(0, Value.Null.Instance)) : Run(sql, parameters);

    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult<IReadOnlyList<Row>>([])
            : Read(sql, parameters);

    public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) =>
        Exec($"BEGIN ISOLATION LEVEL {PostgresBind.IsolationSql(isolation)}");

    public Task Commit() => Exec("COMMIT");
    public Task Rollback() => Exec("ROLLBACK");

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

    NpgsqlConnection RequireDb() => _db ?? throw new DatasourceException("Postgres driver is not open");

    async Task<PostgresExecuteResult> Run(string sql, Row parameters)
    {
        var rewritten = PostgresBind.WithReturning(sql);
        var (text, values) = PostgresBind.ToPostgres(rewritten, parameters);
        using var command = RequireDb().CreateCommand();
        command.CommandText = text;
        for (var i = 0; i < values.Length; i++)
            command.Parameters.AddWithValue($"p{i}", values[i] ?? DBNull.Value);
        if (rewritten.Contains("RETURNING", StringComparison.OrdinalIgnoreCase))
        {
            var result = await command.ExecuteScalarAsync();
            return new PostgresExecuteResult(result is null ? 0 : 1, Value.From(result));
        }
        var rows = await command.ExecuteNonQueryAsync();
        return new PostgresExecuteResult(Math.Max(rows, 0), Value.Null.Instance);
    }

    async Task<IReadOnlyList<Row>> Read(string sql, Row parameters)
    {
        var (text, values) = PostgresBind.ToPostgres(sql, parameters);
        using var command = RequireDb().CreateCommand();
        command.CommandText = text;
        for (var i = 0; i < values.Length; i++)
            command.Parameters.AddWithValue($"p{i}", values[i] ?? DBNull.Value);
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

    async Task Exec(string sql)
    {
        using var command = RequireDb().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
