using MySqlConnector;

namespace Datasources.Mysql;

internal sealed class MysqlConnection : IConnection
{
    MySqlConnection? _db;

    public string Url { get; }
    public string Provider => "mysql";
    public ICommand CreateCommand() =>
        new MysqlCommand(this, new StatementTextRenderer(t => new MysqlStatementRenderer(t)));
    public bool IsOpen { get; private set; }

    public MysqlConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new MysqlConnectionOptions
        {
            Url = DatasourceOptions.Value(options, "DATASOURCE_URL")
        })
    {
    }

    public MysqlConnection(MysqlConnectionOptions? options = null)
    {
        options ??= new MysqlConnectionOptions();
        Url = options.Url ?? "";
    }

    public async Task Open()
    {
        if (IsOpen) throw new DatasourceException("Connection is already open");
        _db = new MySqlConnection(MysqlConnectionString.ToConnectionString(Url));
        await _db.OpenAsync();
        await Exec("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')");
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
        await Exec($"SET TRANSACTION ISOLATION LEVEL {MysqlBind.IsolationSql(isolation)}");
        await Exec("START TRANSACTION");
        return new MysqlTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }

    internal MySqlConnection Require() =>
        _db ?? throw new DatasourceException("Connection is not open");

    internal async Task Exec(string sql)
    {
        using var command = Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }
}
