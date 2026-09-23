using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Json;

internal sealed class JsonConnection : IConnection
{
    public string Path { get; }
    public string? Collection { get; set; }
    public IFileStore FileStore { get; }
    public JsonNode Data { get; set; }
    public string Provider => "json";
    public ICommand CreateCommand() => new JsonCommand(this);
    public bool IsOpen { get; private set; }
    public bool InTransaction { get; private set; }
    private JsonNode? _snapshot;
    private readonly JsonNode? _initial;
    private readonly bool _hasInitial;

    public JsonConnection(IReadOnlyDictionary<string, string?>? options)
        : this(new JsonConnectionOptions { Path = DatasourceOptions.Value(options, "path") ?? "" })
    {
    }

    public JsonConnection(JsonConnectionOptions? options = null)
    {
        options ??= new JsonConnectionOptions();
        Path = options.Path;
        Collection = options.Collection;
        FileStore = options.FileStore ?? new MemoryFileStore();
        _hasInitial = options.Data is not null;
        _initial = options.Data;
        Data = Empty();
    }

    public async Task Open()
    {
        if (IsOpen) return;
        if (_hasInitial)
            Data = _initial?.DeepClone() ?? Empty();
        else if (!string.IsNullOrEmpty(Path) && await FileStore.Exists(Path))
            Data = JsonNode.Parse(await FileStore.Read(Path)) ?? Empty();
        else
            Data = Empty();

        if (Collection is not null) GetCollection();
        IsOpen = true;
    }

    public Task Close()
    {
        IsOpen = false;
        return Task.CompletedTask;
    }

    public Task<ITransaction> StartTransaction(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        EnsureOpen();
        _snapshot = Data.DeepClone();
        InTransaction = true;
        return Task.FromResult<ITransaction>(new JsonTransaction(this, isolation));
    }

    public JsonArray GetCollection()
    {
        if (Collection is not null)
        {
            if (Data is not JsonObject obj)
            {
                Data = new JsonObject();
                obj = (JsonObject)Data;
            }

            if (obj[Collection] is not JsonArray array)
            {
                array = new JsonArray();
                obj[Collection] = array;
            }

            return array;
        }

        if (Data is not JsonArray)
            Data = new JsonArray();

        return (JsonArray)Data;
    }

    public async Task Persist()
    {
        if (InTransaction || string.IsNullOrEmpty(Path)) return;
        await FileStore.Write(Path, Data.ToJsonString());
    }

    public void FinishCommit()
    {
        InTransaction = false;
        _snapshot = null;
    }

    public void FinishAbort()
    {
        Data = _snapshot ?? Empty();
        InTransaction = false;
        _snapshot = null;
    }

    public void EnsureOpen()
    {
        if (!IsOpen) throw new DatasourceException("Connection is not open");
    }

    private JsonNode Empty() => Collection is null ? new JsonArray() : new JsonObject();
}
