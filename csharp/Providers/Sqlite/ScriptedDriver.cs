namespace Datasources.Sqlite;

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
