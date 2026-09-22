namespace Datasources.Postgres;

public sealed class PostgresConnection : IConnection
{

    public string Url { get; }
    public string? Schema { get; }
    public IPostgresDriver Driver { get; }
    public string Provider => "postgres";
    public ICommand CreateCommand() => new PostgresCommand(this);
    public bool IsOpen { get; private set; }

    public PostgresConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new PostgresConnectionOptions
        {
            Url = DatasourceOptions.Value(options, "url")
                ?? DatasourceOptions.Value(options, "database_url"),
            Schema = DatasourceOptions.Value(options, "schema")
                ?? DatasourceOptions.Value(options, "database_schema")
        })
    {
    }

    public PostgresConnection(PostgresConnectionOptions? options = null)
    {
        options ??= new PostgresConnectionOptions();
        Url = options.Url ?? "";
        Schema = options.Schema;
        Driver = PostgresGuard.Require(options.Driver);
    }



    public async Task Open()
    {
        if (IsOpen) return;
        await Driver.Open(Url);
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
        return new PostgresTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }
}
