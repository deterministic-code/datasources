using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Memory;

public sealed class MemoryStore
{
    public Dictionary<string, List<Dictionary<string, object?>>> Collections { get; } = new();

    public List<Dictionary<string, object?>> GetCollection(string name)
    {
        if (!Collections.TryGetValue(name, out var rows))
        {
            rows = [];
            Collections[name] = rows;
        }

        return rows;
    }

    public bool HasCollection(string name) => Collections.ContainsKey(name);

    public Dictionary<string, List<Dictionary<string, object?>>> Snapshot() =>
        Collections.ToDictionary(pair => pair.Key, pair => CloneRows(pair.Value));

    public void Restore(Dictionary<string, List<Dictionary<string, object?>>> snapshot)
    {
        Collections.Clear();
        foreach (var pair in snapshot)
        {
            Collections[pair.Key] = CloneRows(pair.Value);
        }
    }

    public void Seed(object data, string collection)
    {
        if (data is IDictionary<string, List<Dictionary<string, object?>>> named)
        {
            foreach (var pair in named)
            {
                if (!HasCollection(pair.Key))
                {
                    Collections[pair.Key] = CloneRows(pair.Value);
                }
            }

            return;
        }

        if (data is IEnumerable<Dictionary<string, object?>> rows && !HasCollection(collection))
        {
            Collections[collection] = CloneRows(rows);
        }
    }

    public static List<Dictionary<string, object?>> CloneRows(IEnumerable<Dictionary<string, object?>> rows) =>
        rows.Select(row => new Dictionary<string, object?>(row)).ToList();
}

public sealed class MemoryConnectionOptions
{
    public string? Collection { get; set; }
    public MemoryStore? Store { get; set; }
    public object? Data { get; set; }
}

public sealed class MemoryConnection : MiddlewareHost, IConnection
{
    public MemoryStore Store { get; }
    public string Collection { get; set; }
    public bool Opened { get; private set; }
    public bool InTransaction { get; private set; }
    private readonly object? _data;
    private Dictionary<string, List<Dictionary<string, object?>>>? _snapshot;

    public MemoryConnection(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        Store = options.Store ?? new MemoryStore();
        Collection = options.Collection ?? "";
        _data = options.Data;
    }

    public Task Open() => Run("open", () =>
    {
        if (Opened) return Task.CompletedTask;
        if (_data is not null)
        {
            Store.Seed(_data, Collection);
        }

        Store.GetCollection(Collection);
        Opened = true;
        return Task.CompletedTask;
    });

    public Task Close() => Run("close", () =>
    {
        Opened = false;
        return Task.CompletedTask;
    });

    public Task<ITransaction> StartTransaction() => Run("startTransaction", () =>
    {
        EnsureOpen();
        _snapshot = Store.Snapshot();
        InTransaction = true;
        return Task.FromResult<ITransaction>(new MemoryTransaction(this));
    });

    public List<Dictionary<string, object?>> GetCollection() => Store.GetCollection(Collection);

    public void FinishCommit()
    {
        InTransaction = false;
        _snapshot = null;
    }

    public void FinishAbort()
    {
        if (_snapshot is not null)
        {
            Store.Restore(_snapshot);
        }

        InTransaction = false;
        _snapshot = null;
    }

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}

public sealed class MemoryTransaction : MiddlewareHost, ITransaction
{
    public MemoryTransaction(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task Commit() => Run("commit", () =>
    {
        Typed.FinishCommit();
        return Task.CompletedTask;
    });

    public Task Abort() => Run("abort", () =>
    {
        Typed.FinishAbort();
        return Task.CompletedTask;
    });
}

public sealed class MemoryCommand : MiddlewareHost, ICommand
{
    public MemoryCommand(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", () =>
        {
            Typed.EnsureOpen();
            var filter = Merge(ParseFilter(text), Helpers.ParametersToRecord(parameters));
            var rows = Typed.GetCollection().Where(row => Matches(row, filter)).Cast<object?>().ToList();
            return Task.FromResult<IReader>(new ArrayReader(rows));
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", () =>
        {
            Typed.EnsureOpen();
            var op = (text ?? "").Trim().ToLowerInvariant();
            var obj = Helpers.ParametersToRecord(parameters).ToDictionary(k => k.Key, v => v.Value);
            if (op == "insert")
            {
                Typed.GetCollection().Add(new Dictionary<string, object?>(obj));
                return Task.FromResult(new Response(obj, 1));
            }

            if (op == "update")
            {
                var whereText = obj.TryGetValue("$where", out var where) ? where as string : null;
                obj.Remove("$where");
                var filter = whereText is not null ? ParseFilter(whereText) : Helpers.IdentityFilter(obj);
                return Task.FromResult(ApplyUpdate(obj, filter));
            }

            if (op == "delete")
            {
                var filter = obj.Count == 0 ? new Dictionary<string, object?>() : Helpers.IdentityFilter(obj);
                return Task.FromResult(ApplyDelete(filter));
            }

            throw new DatasourceException($"Unsupported Memory command: {text ?? ""}");
        }, text, parameters);

    private Response ApplyUpdate(IReadOnlyDictionary<string, object?> obj, IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var affected = 0;
        foreach (var row in collection.Where(row => Matches(row, filter)))
        {
            foreach (var pair in obj)
            {
                row[pair.Key] = pair.Value;
            }

            affected++;
        }

        return new Response(obj, affected);
    }

    private Response ApplyDelete(IReadOnlyDictionary<string, object?> filter)
    {
        var collection = Typed.GetCollection();
        var remaining = collection.Where(row => !Matches(row, filter)).ToList();
        var affected = collection.Count - remaining.Count;
        collection.Clear();
        collection.AddRange(remaining);
        return new Response(null, affected);
    }

    public static Dictionary<string, object?> ParseFilter(string? text)
    {
        if (string.IsNullOrWhiteSpace(text))
        {
            return new Dictionary<string, object?>();
        }

        var node = JsonNode.Parse(text);
        return node is JsonObject obj
            ? obj.ToDictionary(k => k.Key, v => JsonValueToObject(v.Value))
            : throw new DatasourceException("Memory filter must be an object");
    }

    public static bool Matches(IReadOnlyDictionary<string, object?> row, IReadOnlyDictionary<string, object?> filter) =>
        filter.All(pair => Equals(row.TryGetValue(pair.Key, out var value) ? value : null, pair.Value));

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

public sealed class MemoryRepository : IRepository<Dictionary<string, object?>>
{
    public MemoryRepository(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public async Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        return await reader.Next() ? reader.Current as Dictionary<string, object?> : null;
    }

    public async IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        while (await reader.Next())
        {
            if (reader.Current is Dictionary<string, object?> record)
            {
                yield return record;
            }
        }
    }

    public Task<Response> Add(Dictionary<string, object?> obj)
    {
        Typed.EnsureOpen();
        Typed.GetCollection().Add(new Dictionary<string, object?>(obj));
        return Task.FromResult(new Response(obj, 1));
    }

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("update", ToParams(obj, byWhere));
    }

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("delete", ToParams(obj, byWhere));
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
}

public sealed class MemoryDatasource : IDatasource, IDatasourceFactory
{
    public static readonly MemoryDatasource Factory = new();
    private readonly MemoryConnectionOptions _options;
    public string Provider => "memory";
    public MemoryDatasource(MemoryConnectionOptions? options = null)
    {
        options ??= new MemoryConnectionOptions();
        options.Store ??= new MemoryStore();
        _options = options;
    }

    public IDatasource Create(object? options = null) =>
        new MemoryDatasource(options as MemoryConnectionOptions ?? new MemoryConnectionOptions());

    public IConnection CreateConnection() => new MemoryConnection(_options);

    public ICommand CreateCommand(IConnection? connection = null) =>
        new MemoryCommand(connection as MemoryConnection ?? (MemoryConnection)CreateConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null)
    {
        var conn = connection as MemoryConnection ?? (MemoryConnection)CreateConnection();
        if (options?.Collection is not null)
        {
            conn.Collection = options.Collection;
        }

        return (IRepository<T>)(object)new MemoryRepository(conn);
    }
}
