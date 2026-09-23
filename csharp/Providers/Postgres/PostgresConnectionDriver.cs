using Npgsql;

namespace Datasources.Postgres;

internal sealed class PostgresConnectionDriver(string? schema = null)
{
    NpgsqlConnection? _db;

    public async Task Open(string url)
    {
        if (_db is not null) return;
        _db = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url));
        await _db.OpenAsync();
        if (!string.IsNullOrEmpty(schema))
            await Exec($"SET search_path TO {schema}");
    }

    public async Task Close()
    {
        if (_db is null) return;
        await _db.DisposeAsync();
        _db = null;
    }

    internal NpgsqlConnection Require() =>
        _db ?? throw new DatasourceException("Postgres driver is not open");

    async Task Exec(string sql)
    {
        using var command = Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
