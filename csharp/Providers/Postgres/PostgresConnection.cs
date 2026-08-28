namespace Datasources.Postgres;

public sealed class PostgresConnection : IConnection
{
    public string Url { get; }
    public string? Schema { get; }
    public IPostgresDriver Driver { get; }
    public string Provider => "postgres";
    public ICommand CreateCommand() => new PostgresCommand(this);
    public bool Opened { get; private set; }

    public PostgresConnection(PostgresConnectionOptions? options = null)
    {
        options ??= new PostgresConnectionOptions();
        Url = options.Url ?? "";
        Schema = options.Schema;
        Driver = PostgresGuard.Require(options.Driver);
    }

    public async Task Open()
    {
        if (Opened) return;
        await Driver.Open(Url);
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
        return new PostgresTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}
