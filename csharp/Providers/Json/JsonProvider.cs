using System.Text.Json;
using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Json;

public interface IFileStore
{
    Task<string> Read(string path);
    Task Write(string path, string contents);
    Task<bool> Exists(string path);
}

public sealed class MemoryFileStore : IFileStore
{
    public Dictionary<string, string> Files { get; } = new();
    public Task<string> Read(string path) =>
        Files.TryGetValue(path, out var contents)
            ? Task.FromResult(contents)
            : throw new DatasourceException($"File not found: {path}");
    public Task Write(string path, string contents)
    {
        Files[path] = contents;
        return Task.CompletedTask;
    }
    public Task<bool> Exists(string path) => Task.FromResult(Files.ContainsKey(path));
}

public sealed class JsonConnectionOptions
{
    public string Path { get; set; } = "";
    public string? Collection { get; set; }
    public IFileStore? FileStore { get; set; }
    public JsonNode? Data { get; set; }
}

public sealed class JsonConnection : MiddlewareHost, IConnection
{
    public string Path { get; }
    public string? Collection { get; set; }
    public IFileStore FileStore { get; }
    public JsonNode Data { get; set; }
    public bool Opened { get; private set; }
    public bool InTransaction { get; private set; }
    private JsonNode? _snapshot;
    private readonly JsonNode? _initial;
    private readonly bool _hasInitial;

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

    public Task Open() => Run("open", async () =>
    {
        if (Opened) return;
        if (_hasInitial)
        {
            Data = _initial?.DeepClone() ?? Empty();
        }
        else if (!string.IsNullOrEmpty(Path) && await FileStore.Exists(Path))
        {
            Data = JsonNode.Parse(await FileStore.Read(Path)) ?? Empty();
        }
        else
        {
            Data = Empty();
        }

        if (Collection is not null)
        {
            GetCollection();
        }

        Opened = true;
    });

    public Task Close() => Run("close", () =>
    {
        Opened = false;
        return Task.CompletedTask;
    });

    public Task<ITransaction> StartTransaction() => Run("startTransaction", () =>
    {
        EnsureOpen();
        _snapshot = Data.DeepClone();
        InTransaction = true;
        return Task.FromResult<ITransaction>(new JsonTransaction(this));
    });

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

        if (Data is not JsonArray root)
        {
            Data = new JsonArray();
        }

        return (JsonArray)Data;
    }

    public async Task Persist()
    {
        if (InTransaction || string.IsNullOrEmpty(Path))
        {
            return;
        }

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
        if (!Opened) throw new DatasourceException("Connection is not open");
    }

    private JsonNode Empty() => Collection is null ? new JsonArray() : new JsonObject();
}

public sealed class JsonTransaction : MiddlewareHost, ITransaction
{
    public JsonTransaction(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task Commit() => Run("commit", async () =>
    {
        Typed.FinishCommit();
        if (!string.IsNullOrEmpty(Typed.Path))
        {
            await Typed.FileStore.Write(Typed.Path, Typed.Data.ToJsonString());
        }
    });

    public Task Abort() => Run("abort", () =>
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    });
}

public sealed class JsonCommand : MiddlewareHost, ICommand
{
    public JsonCommand(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", () =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(text), Helpers.ParametersToRecord(parameters));
            var rows = Typed.GetCollection().Where(row => Matches(row, filter)).Cast<object?>().ToList();
            return Task.FromResult<IReader>(new ArrayReader(rows));
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", async () =>
        {
            Typed.EnsureOpen();
            var op = (text ?? "").Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(ToObject(obj));
                await Typed.Persist();
                return new Response(obj, 1);
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) ? where as string : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : Helpers.IdentityFilter(obj);
                return await ApplyUpdate(obj, filter);
            }

            if (op == "delete")
            {
                var filter = obj.Count == 0 ? new Dictionary<string, object?>() : Helpers.IdentityFilter(obj);
                return await ApplyDelete(filter);
            }

            throw new DatasourceException($"Unsupported JSON command: {text ?? ""}");
        }, text, parameters);

    private async Task<Response> ApplyUpdate(IReadOnlyDictionary<string, object?> obj, IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        for (var i = 0; i < collection.Count; i++)
        {
            if (Matches(collection[i], filter) && collection[i] is JsonObject current)
            {
                foreach (var pair in obj)
                {
                    current[pair.Key] = JsonValue.Create(pair.Value);
                }

                affected++;
            }
        }

        await Typed.Persist();
        return new Response(obj, affected);
    }

    private async Task<Response> ApplyDelete(IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var remaining = collection.Where(row => !Matches(row, filter)).Select(n => n?.DeepClone()).ToList();
        var affected = collection.Count - remaining.Count;
        collection.Clear();
        foreach (var row in remaining)
        {
            collection.Add(row);
        }

        await Typed.Persist();
        return new Response(null, affected);
    }

    public static Dictionary<string, object?> ParseFilter(string? text)
    {
        if (string.IsNullOrWhiteSpace(text))
        {
            return new Dictionary<string, object?>();
        }

        var node = JsonNode.Parse(text);
        if (node is not JsonObject obj)
        {
            throw new DatasourceException("JSON filter must be an object");
        }

        return obj.ToDictionary(k => k.Key, v => JsonValueToObject(v.Value));
    }

    public static bool Matches(JsonNode? row, IReadOnlyDictionary<string, object?> filter)
    {
        if (row is not JsonObject obj)
        {
            return false;
        }

        return filter.All(pair => Equals(JsonValueToObject(obj[pair.Key]), pair.Value));
    }

    public static JsonObject ToObject(IReadOnlyDictionary<string, object?> obj)
    {
        var json = new JsonObject();
        foreach (var pair in obj)
        {
            json[pair.Key] = JsonValue.Create(pair.Value);
        }

        return json;
    }

    private static Dictionary<string, object?> Merge(Dictionary<string, object?> left, IReadOnlyDictionary<string, object?> right)
    {
        foreach (var pair in right)
        {
            left[pair.Key] = pair.Value;
        }

        return left;
    }

    public static object? JsonValueToObject(JsonNode? node)
    {
        if (node is not JsonValue value)
        {
            return node;
        }

        if (value.TryGetValue<int>(out var i)) return i;
        if (value.TryGetValue<long>(out var l)) return l;
        if (value.TryGetValue<bool>(out var b)) return b;
        if (value.TryGetValue<double>(out var d)) return d;
        if (value.TryGetValue<string>(out var s)) return s;
        return value.ToString();
    }
}

public sealed class JsonRepository : IRepository<Dictionary<string, object?>>
{
    public JsonRepository(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public async Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null)
    {
        var reader = await new JsonCommand(Typed).ExecuteReader(query?.GetText());
        return await reader.Next() ? ToRecord(reader.Current) : null;
    }

    public async IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null)
    {
        var reader = await new JsonCommand(Typed).ExecuteReader(query?.GetText());
        while (await reader.Next())
        {
            var record = ToRecord(reader.Current);
            if (record is not null)
            {
                yield return record;
            }
        }
    }

    public async Task<Response> Add(Dictionary<string, object?> obj)
    {
        Typed.EnsureOpen();
        Typed.GetCollection().Add(JsonCommand.ToObject(obj));
        await Typed.Persist();
        return new Response(obj, 1);
    }

    public async Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        var filter = byWhere is null ? Helpers.IdentityFilter(obj) : JsonCommand.ParseFilter(byWhere.GetText());
        var command = new JsonCommand(Typed);
        return await command.Execute("update", ToParams(obj, byWhere));
    }

    public async Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return await new JsonCommand(Typed).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, object?> obj, IWhere? byWhere)
    {
        var list = obj.Select(p => new Parameter(p.Key, p.Value)).ToList();
        if (byWhere is not null)
        {
            list.Add(new Parameter("$where", byWhere.GetText()));
        }

        return list;
    }

    private static Dictionary<string, object?>? ToRecord(object? current)
    {
        if (current is JsonObject obj)
        {
            return obj.ToDictionary(k => k.Key, v => JsonCommand.JsonValueToObject(v.Value));
        }

        return null;
    }
}

public sealed class JsonDatasource : IDatasource, IDatasourceFactory
{
    public static readonly JsonDatasource Factory = new();
    private readonly JsonConnectionOptions _options;
    public string Provider => "json";
    public JsonDatasource(JsonConnectionOptions? options = null) => _options = options ?? new JsonConnectionOptions();
    public IDatasource Create(object? options = null) => new JsonDatasource(options as JsonConnectionOptions ?? new JsonConnectionOptions());
    JsonConnection NewConnection() => new(_options);
    public Task<IConnection> CreateConnection() => Task.FromResult<IConnection>(NewConnection());
    public ICommand CreateCommand(IConnection? connection = null) =>
        new JsonCommand(connection as JsonConnection ?? NewConnection());
    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null)
    {
        var conn = connection as JsonConnection ?? NewConnection();
        if (options?.Collection is not null)
        {
            conn.Collection = options.Collection;
        }

        return (IRepository<T>)(object)new JsonRepository(conn);
    }
}
