using MySqlConnector;

namespace Datasources.Mysql;

public sealed class MysqlDriver : IMysqlDriver
{
    MySqlConnection? _db;

    public async Task Open(string url)
    {
        if (_db is not null) return;
        _db = new MySqlConnection(ToConnectionString(url));
        await _db.OpenAsync();
        await Exec("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')");
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    public Task<MysqlExecuteResult> Execute(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? Task.FromResult(new MysqlExecuteResult(0, Value.Null.Instance)) : Run(sql, parameters);

    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult<IReadOnlyList<Row>>([])
            : Read(sql, parameters);

    public async Task Begin(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        await Exec($"SET TRANSACTION ISOLATION LEVEL {MysqlBind.IsolationSql(isolation)}");
        await Exec("START TRANSACTION");
    }

    public Task Commit() => Exec("COMMIT");
    public Task Rollback() => Exec("ROLLBACK");

    public static string ToConnectionString(string url)
    {
        if (!url.StartsWith("mysql://", StringComparison.OrdinalIgnoreCase)) return url;
        var uri = new Uri(url);
        var user = uri.UserInfo.Split(':');
        var port = uri.Port > 0 ? uri.Port : 3306;
        var database = uri.AbsolutePath.Trim('/');
        var password = user.Length > 1 ? Uri.UnescapeDataString(user[1]) : "";
        return $"Server={uri.Host};Port={port};User ID={Uri.UnescapeDataString(user[0])};Password={password};Database={database}";
    }

    MySqlConnection RequireDb() => _db ?? throw new DatasourceException("MySQL driver is not open");

    async Task<MysqlExecuteResult> Run(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        var rows = await command.ExecuteNonQueryAsync();
        return new MysqlExecuteResult(rows, Value.From(command.LastInsertedId));
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

    MySqlCommand Bind(string sql, Row parameters)
    {
        var command = RequireDb().CreateCommand();
        command.CommandText = MysqlBind.ToMysql(sql);
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '@' or ':' or '$' ? name.TrimStart('$', '@', ':') : name;
            command.Parameters.AddWithValue($"@{key}", value.ToObject() ?? DBNull.Value);
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
