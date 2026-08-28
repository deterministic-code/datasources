using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteConnection : MiddlewareHost, IConnection
{
    public string Filename { get; }
    public ISqliteDriver Driver { get; }
    public bool Opened { get; private set; }

    public SqliteConnection(SqliteConnectionOptions? options = null)
    {
        options ??= new SqliteConnectionOptions();
        Filename = options.Filename ?? ":memory:";
        Driver = options.Driver ?? new SqliteDriver();
    }

    public Task Open() => Run("open", async () =>
    {
        if (Opened) return;
        await Driver.Open(Filename);
        Opened = true;
    });

    public Task Close() => Run("close", async () =>
    {
        await Driver.Close();
        Opened = false;
    });

    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable) =>
        Run("startTransaction", async () =>
        {
            EnsureOpen();
            await Driver.Begin(isolation);
            return (ITransaction)new SqliteTransaction(this, isolation);
        });

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}
