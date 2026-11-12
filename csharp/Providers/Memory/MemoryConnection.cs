using Datasources;

namespace Datasources.Memory;

internal sealed class MemoryConnection : IConnection
{
    readonly object? _data;

    public MemoryEngine Engine { get; }
    public string Collection
    {
        get => Engine.Collection;
        set => Engine.Collection = value;
    }
    public string Provider => "memory";
    public ICommand CreateCommand() => new MemoryCommand(this);
    public bool IsOpen { get; private set; }
    public bool InTransaction => Engine.InTransaction;

    public MemoryConnection(IReadOnlyDictionary<string, string?>? _)
        : this(new MemoryConnectionOptions())
    {
    }

    public MemoryConnection(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        Engine = options.Engine ?? new MemoryEngine { Collection = options.Collection ?? "" };
        if (options.Collection is not null) Engine.Collection = options.Collection;
        _data = options.Data;
    }

    public Task Open()
    {
        if (IsOpen) throw new DatasourceException("Connection is already open");
        if (_data is not null) Engine.Seed(_data, Collection);
        Engine.GetCollection(Collection);
        IsOpen = true;
        return Task.CompletedTask;
    }

    public Task Close()
    {
        IsOpen = false;
        return Task.CompletedTask;
    }

    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        EnsureOpen();
        Engine.Begin();
        return Task.FromResult<ITransaction>(new MemoryTransaction(this, isolation));
    }

    public List<Row> GetCollection() => Engine.GetCollection(Collection);

    public void FinishCommit() => Engine.Commit();

    public void FinishAbort() => Engine.Abort();

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }
}
