using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteConnection : IConnection
{
    public string Filename { get; }
    public ISqliteDriver Driver { get; }
    public bool Opened { get; private set; }

    public SqliteConnection(SqliteConnectionOptions? options = null)
    {
        options ??= new SqliteConnectionOptions();
        Filename = options.Filename ?? ":memory:";
        Driver = SqliteGuard.Require(options.Driver);
    }

    public async Task Open()
    {
        if (Opened) return;
        await Driver.Open(Filename);
        Opened = true;
    }

    public async Task Close()
    {
        await Driver.Close();
        Opened = false;
    }

    public async Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        EnsureOpen();
        await Driver.Begin(isolation);
        return new SqliteTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}
