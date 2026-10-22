using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteConnection : IConnection
{
    readonly ISqliteConnectionSession _session;
    readonly ISqliteCommands _commands;
    readonly ISqliteTransactions _transactions;

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
        Filename = options.Filename ?? SqlitePath.FromUrl(options.Url) ?? ":memory:";
        var session = new SqliteConnectionDriver();
        var commands = new SqliteCommandDriver(session);
        _session = session;
        _commands = commands;
        _transactions = new SqliteTransactionDriver(commands);
    }

    public async Task Open()
    {
        if (IsOpen) throw new DatasourceException("Connection is already open");
        await _session.Open(Filename);
        IsOpen = true;
    }

    public async Task Close()
    {
        await _session.Close();
        IsOpen = false;
    }

    public async Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        EnsureOpen();
        await _transactions.Begin(isolation);
        return new SqliteTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }

    internal ISqliteCommands Commands => _commands;
    internal ISqliteTransactions Transactions => _transactions;
}
