using Datasources;

namespace Datasources.Sqlite;

public sealed record SqliteExecuteResult(int RowsAffected, object? Result);

public interface ISqliteDriver
{
    Task Open(string filename);
    Task Close();
    Task<SqliteExecuteResult> Execute(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task<IReadOnlyList<IReadOnlyDictionary<string, object?>>> Query(string sql, IReadOnlyDictionary<string, object?> parameters);
    Task Begin();
    Task Commit();
    Task Rollback();
}

public static class SqliteGuard
{
    public static ISqliteDriver Require(ISqliteDriver? driver) =>
        driver ?? throw new DatasourceException("SQLite driver is required");
}

public sealed class ScriptedDriver : ISqliteDriver
{
    public bool Opened { get; private set; }
    public bool Closed { get; private set; }
    public int Begins { get; private set; }
    public int Commits { get; private set; }
    public int Rollbacks { get; private set; }
    public List<string> Executes { get; } = new();
    public List<string> Queries { get; } = new();
    public List<IReadOnlyDictionary<string, object?>> QueryResult { get; set; } = new();
    public SqliteExecuteResult ExecuteResult { get; set; } = new(1, 10);

    public Task Open(string filename)
    {
        Opened = true;
        return Task.CompletedTask;
    }

    public Task Close()
    {
        Closed = true;
        return Task.CompletedTask;
    }

    public Task<SqliteExecuteResult> Execute(string sql, IReadOnlyDictionary<string, object?> parameters)
    {
        Executes.Add(sql);
        return Task.FromResult(ExecuteResult);
    }

    public Task<IReadOnlyList<IReadOnlyDictionary<string, object?>>> Query(string sql, IReadOnlyDictionary<string, object?> parameters)
    {
        Queries.Add(sql);
        return Task.FromResult<IReadOnlyList<IReadOnlyDictionary<string, object?>>>(QueryResult);
    }

    public Task Begin()
    {
        Begins++;
        return Task.CompletedTask;
    }

    public Task Commit()
    {
        Commits++;
        return Task.CompletedTask;
    }

    public Task Rollback()
    {
        Rollbacks++;
        return Task.CompletedTask;
    }
}

public sealed class SqliteConnectionOptions
{
    public string? Filename { get; set; }
    public ISqliteDriver? Driver { get; set; }
}

public sealed class SqliteConnection : MiddlewareHost, IConnection
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

    public Task Open() => Run("open", async () =>
    {
        if (Opened) return;
        await Driver.Open(Filename);
        Opened = true;
    });

    public Task Close() => Run("close", async () =>
    {
        await Driver.Close();
        Opened = false;
    });

    public Task<ITransaction> StartTransaction() => Run("startTransaction", async () =>
    {
        EnsureOpen();
        await Driver.Begin();
        return (ITransaction)new SqliteTransaction(this);
    });

    public void EnsureOpen()
    {
        if (!Opened) throw new DatasourceException("Connection is not open");
    }
}

public sealed class SqliteTransaction : MiddlewareHost, ITransaction
{
    public SqliteTransaction(SqliteConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    public Task Commit() => Run("commit", () => ((SqliteConnection)Connection).Driver.Commit());
    public Task Abort() => Run("abort", () => ((SqliteConnection)Connection).Driver.Rollback());
}

public sealed class SqliteCommand : MiddlewareHost, ICommand
{
    public SqliteCommand(SqliteConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private SqliteConnection Typed => (SqliteConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", async () =>
        {
            Typed.EnsureOpen();
            var rows = await Typed.Driver.Query(text ?? "", Helpers.ParametersToRecord(parameters));
            return (IReader)new ArrayReader(rows.Cast<object?>().ToList());
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", async () =>
        {
            Typed.EnsureOpen();
            var result = await Typed.Driver.Execute(text ?? "", Helpers.ParametersToRecord(parameters));
            return new Response(result.Result, result.RowsAffected);
        }, text, parameters);
}

public sealed class SqliteRepository : IRepository<Dictionary<string, object?>>
{
    public string Table { get; }
    public IConnection Connection { get; }
    private SqliteConnection Typed => (SqliteConnection)Connection;

    public SqliteRepository(SqliteConnection connection, string? table = null)
    {
        Connection = connection;
        Table = table ?? "items";
    }

    public async Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null)
    {
        var reader = await new SqliteCommand(Typed).ExecuteReader(SelectSql(query, true));
        if (!await reader.Next())
        {
            return null;
        }

        return (Dictionary<string, object?>?)ToDict(reader.Current);
    }

    public async IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null)
    {
        var reader = await new SqliteCommand(Typed).ExecuteReader(SelectSql(query, false));
        while (await reader.Next())
        {
            if (ToDict(reader.Current) is { } record)
            {
                yield return record;
            }
        }
    }

    public Task<Response> Add(Dictionary<string, object?> obj)
    {
        var keys = obj.Keys.ToList();
        var sql = keys.Count == 0
            ? $"INSERT INTO {Table} DEFAULT VALUES"
            : $"INSERT INTO {Table} ({string.Join(", ", keys)}) VALUES ({string.Join(", ", keys.Select(k => $"${k}"))})";
        return new SqliteCommand(Typed).Execute(sql, ToParams(obj));
    }

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        var keys = obj.Keys.ToList();
        var assignments = keys.Count == 0 ? "id = id" : string.Join(", ", keys.Select(k => $"{k} = ${k}"));
        var where = byWhere?.GetText() ?? Helpers.IdentityWhereText(obj);
        return new SqliteCommand(Typed).Execute($"UPDATE {Table} SET {assignments} WHERE {where}", ToParams(obj));
    }

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        var where = byWhere?.GetText() ?? Helpers.IdentityWhereText(obj);
        return new SqliteCommand(Typed).Execute($"DELETE FROM {Table} WHERE {where}", ToParams(obj));
    }

    private string SelectSql(IQuery? query, bool single)
    {
        var where = query is null ? "" : $" WHERE {query.GetText()}";
        var limit = single ? " LIMIT 1" : "";
        return $"SELECT * FROM {Table}{where}{limit}";
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, object?> obj) =>
        obj.Select(p => new Parameter(p.Key, p.Value)).ToList();

    private static Dictionary<string, object?>? ToDict(object? current) =>
        current as Dictionary<string, object?> ??
        (current as IReadOnlyDictionary<string, object?>)?.ToDictionary(k => k.Key, v => v.Value);
}

public sealed class SqliteDatasource : IDatasource, IDatasourceFactory
{
    public static readonly SqliteDatasource Factory = new(new SqliteConnectionOptions { Driver = new ScriptedDriver() });
    private readonly SqliteConnectionOptions _options;
    private readonly string? _table;
    public string Provider => "sqlite";

    public SqliteDatasource(SqliteConnectionOptions? options = null, string? table = null)
    {
        _options = options ?? new SqliteConnectionOptions();
        _table = table;
    }

    public IDatasource Create(object? options = null) =>
        new SqliteDatasource(options as SqliteConnectionOptions ?? new SqliteConnectionOptions());

    public IConnection CreateConnection() => new SqliteConnection(_options);

    public ICommand CreateCommand(IConnection? connection = null) =>
        new SqliteCommand(connection as SqliteConnection ?? (SqliteConnection)CreateConnection());

    public IRepository<T> CreateRepository<T>(IConnection? connection = null, RepositoryOptions? options = null)
    {
        var conn = connection as SqliteConnection ?? (SqliteConnection)CreateConnection();
        return (IRepository<T>)(object)new SqliteRepository(conn, options?.Table ?? _table);
    }
}
