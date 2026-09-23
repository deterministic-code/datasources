using MySqlConnector;

namespace Datasources.Mysql;

internal sealed class MysqlConnectionDriver
{
    MySqlConnection? _db;

    public async Task Open(string url)
    {
        if (_db is not null) return;
        _db = new MySqlConnection(MysqlConnectionString.ToConnectionString(url));
        await _db.OpenAsync();
        await Exec("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')");
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    internal MySqlConnection Require() =>
        _db ?? throw new DatasourceException("MySQL driver is not open");

    async Task Exec(string sql)
    {
        using var command = Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
