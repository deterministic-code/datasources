namespace Datasources.Mysql;

public sealed class MysqlConnection : IConnection
{
    static MysqlConnection() => _ = MysqlTypes.All;

    public string Url { get; }
    public IMysqlDriver Driver { get; }
    public string Provider => "mysql";
    public ICommand CreateCommand() => new MysqlCommand(this);
    public bool IsOpen { get; private set; }

    public MysqlConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new MysqlConnectionOptions
        {
            Url = DatasourceOptions.Value(options, "url")
                ?? DatasourceOptions.Value(options, "database_url")
        })
    {
    }

    public MysqlConnection(MysqlConnectionOptions? options = null)
    {
        options ??= new MysqlConnectionOptions();
        Url = options.Url ?? "";
        Driver = MysqlGuard.Require(options.Driver);
    }

    public static object Repository(IConnection connection) =>
        new Repository<Dictionary<string, Value>>(
            new RepositoryBuilderOptions<Dictionary<string, Value>>
            {
                Connection = (MysqlConnection)connection,
                Entity = "items",
                Mapping = new RowTypeConverter<Dictionary<string, Value>>(),
            });

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
        return new MysqlTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }
}
