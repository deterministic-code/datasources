using Npgsql;

namespace Datasources.Postgres;

internal sealed class PostgresConnection : IConnection
{
    NpgsqlConnection? _db;

    public string Url { get; }
    public string? Schema { get; }
    public string Provider => "postgres";
    public ICommand CreateCommand() => new PostgresCommand(this);
    public bool IsOpen { get; private set; }

    public PostgresConnection(IReadOnlyDictionary<string, string?>? options)
        : this(Options(options))
    {
    }

    static PostgresConnectionOptions Options(IReadOnlyDictionary<string, string?>? options)
    {
        var url = DatasourceOptions.Value(options, "DATASOURCE_URL");
        return new PostgresConnectionOptions
        {
            Url = url,
            Schema = DatasourceUrlParser.Parse(url)?.Schema
        };
    }

    public PostgresConnection(PostgresConnectionOptions? options = null)
    {
        options ??= new PostgresConnectionOptions();
        Url = options.Url ?? "";
        Schema = options.Schema;
    }

    public async Task Open()
    {
        if (IsOpen) throw new DatasourceException("Connection is already open");
        _db = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(Url));
        await _db.OpenAsync();
        if (!string.IsNullOrEmpty(Schema))
            await Exec($"SET search_path TO {Schema}");
        IsOpen = true;
    }

    public async Task Close()
    {
        if (_db is not null)
            await _db.DisposeAsync();
        _db = null;
        IsOpen = false;
    }

    public async Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        EnsureOpen();
        await Exec($"BEGIN ISOLATION LEVEL {PostgresBind.IsolationSql(isolation)}");
        return new PostgresTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }

    internal NpgsqlConnection Require() =>
        _db ?? throw new DatasourceException("Connection is not open");

    internal async Task Exec(string sql)
    {
        using var command = Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
