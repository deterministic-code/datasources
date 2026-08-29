using Datasources;

namespace Datasources.Memory;

public sealed class MemoryConnection : IConnection
{
    public MemoryStore Store { get; }
    public string Collection { get; set; }
    public string Provider => "memory";
    public ICommand CreateCommand() => new MemoryCommand(this);
    public bool Opened { get; private set; }
    public bool InTransaction { get; private set; }
    private readonly object? _data;
    private Dictionary<string, List<Dictionary<string, Value>>>? _snapshot;

    public MemoryConnection(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        Store = options.Store ?? new MemoryStore();
        Collection = options.Collection ?? "";
        _data = options.Data;
    }

    public Task Open()
    {
        if (Opened) return Task.CompletedTask;
        if (_data is not null) Store.Seed(_data, Collection);
        Store.GetCollection(Collection);
        Opened = true;
        return Task.CompletedTask;
    }

    public Task Close()
    {
        Opened = false;
        return Task.CompletedTask;
    }

    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        EnsureOpen();
        _snapshot = Store.Snapshot();
        InTransaction = true;
        return Task.FromResult<ITransaction>(new MemoryTransaction(this, isolation));
    }

    public List<Dictionary<string, Value>> GetCollection() => Store.GetCollection(Collection);

    public void FinishCommit()
    {
        InTransaction = false;
        _snapshot = null;
    }

    public void FinishAbort()
    {
        if (_snapshot is not null) Store.Restore(_snapshot);
        InTransaction = false;
        _snapshot = null;
    }

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}
