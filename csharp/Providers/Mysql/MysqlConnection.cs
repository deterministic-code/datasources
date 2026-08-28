namespace Datasources.Mysql;

public sealed class MysqlConnection : IConnection
{
    public string Url { get; }
    public IMysqlDriver Driver { get; }
    public string Provider => "mysql";
    public ICommand CreateCommand() => new MysqlCommand(this);
    public bool Opened { get; private set; }

    public MysqlConnection(MysqlConnectionOptions? options = null)
    {
        options ??= new MysqlConnectionOptions();
        Url = options.Url ?? "";
        Driver = MysqlGuard.Require(options.Driver);
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
        return new MysqlTransaction(this, isolation);
    }

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}
