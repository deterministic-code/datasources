using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteConnection : IConnection
{
    global::Microsoft.Data.Sqlite.SqliteConnection? _db;
    readonly SqliteOptions _options;
    internal readonly SemaphoreSlim Gate = new(1, 1);

    public string Filename { get; }
    public string Provider => "sqlite";
    public ICommand CreateCommand() =>
        new SqliteCommand(this, new StatementTextRenderer(t => new SqliteStatementRenderer(t)));
    public bool IsOpen { get; private set; }

    public SqliteConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new SqliteConnectionOptions
        {
            Url = DatasourceOptions.Value(options, "DATASOURCE_URL")
        })
    {
    }

    public SqliteConnection(SqliteConnectionOptions? options = null)
    {
        options ??= new SqliteConnectionOptions();
        var parsed = options.Filename is null ? SqlitePath.Parse(options.Url) : null;
        Filename = options.Filename ?? parsed?.Filename ?? ":memory:";
        _options = parsed?.Options ?? SqliteOptions.Empty;
    }

    public async Task Open()
    {
        if (_options.Error is not null) throw new DatasourceException(_options.Error);
        if (IsOpen) throw new DatasourceException("Connection is already open");
        _db = new global::Microsoft.Data.Sqlite.SqliteConnection(SqliteConnectionString.ToConnectionString(Filename));
        _db.DefaultTimeout = _options.ResolvedBusyTimeout();
        await _db.OpenAsync();
        var journal = _options.ResolvedJournalMode(Filename);
        if (journal is not null && !SqliteOptions.IsMemory(Filename))
            await Exec($"PRAGMA journal_mode = {journal}");
        await Exec($"PRAGMA busy_timeout = {_options.ResolvedBusyTimeout()}");
        if (_options.Synchronous is not null)
            await Exec($"PRAGMA synchronous = {_options.Synchronous}");
        IsOpen = true;
    }

    public async Task Close()
    {
        if (_db is not null)
            await _db.DisposeAsync();
        _db = null;
        IsOpen = false;
    }

    public async Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        EnsureOpen();
        await Exec(isolation == IsolationLevel.ReadUncommitted
            ? "PRAGMA read_uncommitted = 1"
            : "PRAGMA read_uncommitted = 0");
        await Exec("BEGIN");
        return new SqliteTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }

    internal global::Microsoft.Data.Sqlite.SqliteConnection Require() =>
        _db ?? throw new DatasourceException("Connection is not open");

    internal async Task Exec(string sql)
    {
        using var command = Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
