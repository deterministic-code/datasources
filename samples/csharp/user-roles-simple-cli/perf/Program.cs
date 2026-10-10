using System.Diagnostics;
using System.Text.Json;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using UserRolesSimpleCli;


var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var strategyName = Environment.GetEnvironmentVariable("PERF_STRATEGY") ?? "batch";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "1000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? "sqlite:///:memory:";
var connection = provider switch
{
    "sqlite" => new SqliteDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    "mysql" => new MysqlDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    "postgres" => new PostgresDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = url }),
    _ => throw new InvalidOperationException(provider)
};
await connection.Open();
try
{
    var command = connection.CreateCommand();
    await Migrate.Apply(command);
    if (provider is "mysql" or "postgres")
        foreach (var sql in File.ReadAllText(Path.Combine(Shared("user-roles-simple-cli-shared"), $"bulk-{provider}.sql"))
            .Split("---", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(Native.N(sql));
    IBulkWriteStrategy<User>? strategy = strategyName switch
    {
        "single" => null,
        "batch" => new BatchCommandIBulkWriteStrategy<User>(),
        "procedure" => new StoredProcedureBulkWriteStrategy<User>("bulk_create_user", "bulk_update_user", "bulk_delete_user"),
        _ => throw new InvalidOperationException(strategyName)
    };
    var usersRepo = strategy is null
        ? new BulkCRUDRepository<User>(connection, new() { Entity = "users" })
        : new BulkCRUDRepository<User>(connection, new() { Entity = "users" }, strategy);
    var created = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    var users = Enumerable.Range(1, rows).Select(id => new User
    {
        id = strategyName == "batch" ? id : 0,
        username = $"user{id}",
        password = "hash",
        is_enabled = true,
        created = created,
        updated = [1, 2, 3]
    }).ToList();
    var createMs = await Time(() => Each(users, batch => usersRepo.Create(batch)));
    if (await Count(connection, "users") != rows) throw new InvalidOperationException("create count");
    foreach (var user in users) user.is_enabled = false;
    var updateMs = await Time(() => Each(users, batch => usersRepo.Update(batch)));
    Console.WriteLine(JsonSerializer.Serialize(new
    {
        sample = "user-roles-simple-cli",
        language = "csharp",
        provider,
        strategy = strategyName,
        rows,
        chunk = strategyName == "batch" ? chunk : rows,
        createMs,
        updateMs,
        createPerSec = rows / (createMs / 1000),
        updatePerSec = rows / (updateMs / 1000),
        counts = new { users = rows }
    }));
}
finally
{
    try { await connection.Close(); } catch { }
}

async Task Each<T>(List<T> items, Func<List<T>, Task> write)
{
    if (strategyName != "batch") { await write(items); return; }
    for (var at = 0; at < items.Count; at += chunk)
        await write(items.GetRange(at, Math.Min(chunk, items.Count - at)));
}

static async Task<double> Time(Func<Task> work)
{
    var clock = Stopwatch.StartNew();
    await work();
    return clock.Elapsed.TotalMilliseconds;
}

static async Task<int> Count(IConnection connection, string table)
{
    var reader = await connection.CreateCommand().ExecuteReader(Native.N($"SELECT COUNT(*) AS n FROM {table}"));
    try
    {
        await reader.Next();
        return Convert.ToInt32(reader.Current!["n"].ToObject());
    }
    finally
    {
        if (reader is IAsyncDisposable disposable) await disposable.DisposeAsync();
    }
}

static string Shared(string name)
{
    for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
    {
        var path = Path.Combine(dir.FullName, "samples", name);
        if (Directory.Exists(path)) return path;
    }
    throw new InvalidOperationException(name);
}

sealed class User
{
    public int id { get; set; }
    public string username { get; set; } = "";
    public string password { get; set; } = "";
    public bool is_enabled { get; set; }
    public DateTime created { get; set; }
    public byte[] updated { get; set; } = [];
}
