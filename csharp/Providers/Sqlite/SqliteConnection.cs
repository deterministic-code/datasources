using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteConnection : IConnection
{

    public string Filename { get; }
    public ISqliteDriver Driver { get; }
    public string Provider => "sqlite";
    public ICommand CreateCommand() => new SqliteCommand(this);
    public bool IsOpen { get; private set; }

    public SqliteConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new SqliteConnectionOptions
        {
            Filename = DatasourceOptions.Value(options, "filename"),
            Url = DatasourceOptions.Value(options, "url")
                ?? DatasourceOptions.Value(options, "database_url"),
            Driver = SqliteGuard.Engine()
        })
    {
    }

    public SqliteConnection(SqliteConnectionOptions? options = null)
    {
        options ??= new SqliteConnectionOptions();
        Filename = options.Filename ?? SqlitePath.FromUrl(options.Url) ?? ":memory:";
        Driver = SqliteGuard.Require(options.Driver);
    }



    public async Task Open()
    {
        if (IsOpen) return;
        await Driver.Open(Filename);
        IsOpen = true;
    }

    public async Task Close()
    {
        await Driver.Close();
        IsOpen = false;
    }

    public async Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        EnsureOpen();
        await Driver.Begin(isolation);
        return new SqliteTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }
}
