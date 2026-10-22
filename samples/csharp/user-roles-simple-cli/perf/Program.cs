using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using UserRolesSimpleCli;


var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var strategyName = Environment.GetEnvironmentVariable("PERF_STRATEGY") ?? "batch";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "1000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? $"sqlite:///{Path.Combine(Path.GetTempPath(), "user-roles-perf.sqlite")}";
var batched = strategyName == "batch" || strategyName == "procedure" && provider == "sqlite";
Migraters.CreateAndUp(provider, url);
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
    IBulkWriteStrategy<User>? strategy = strategyName switch
    {
        "single" => null,
        _ when batched => new BatchCommandIBulkWriteStrategy<User>(),
        "procedure" => new StoredProcedureBulkWriteStrategy<User>("bulk_create_user", "bulk_update_user", "bulk_delete_user"),
        _ => throw new InvalidOperationException(strategyName)
    };
    var usersRepo = strategy is null
        ? new BulkCRUDRepository<User>(connection, new() { Entity = "users" })
        : new BulkCRUDRepository<User>(connection, new() { Entity = "users" }, strategy);
    var users = LoadUsers();
    var createMs = await Time(() => Each(users, batch => usersRepo.Create(batch)));
    if (await Count(connection, "users") != users.Count) throw new InvalidOperationException("create count");
    foreach (var user in users) user.is_enabled = false;
    var updateMs = await Time(() => Each(users, batch => usersRepo.Update(batch)));
    Console.WriteLine(JsonSerializer.Serialize(new
    {
        sample = "user-roles-simple-cli",
        language = "csharp",
        provider,
        strategy = strategyName,
        rows = users.Count,
        chunk = batched ? Math.Min(chunk, users.Count) : users.Count,
        createMs,
        updateMs,
        createPerSec = users.Count / (createMs / 1000),
        updatePerSec = users.Count / (updateMs / 1000),
        counts = new { users = users.Count }
    }));
}
finally
{
    try { await connection.Close(); } catch { }
}

List<User> LoadUsers()
{
    var path = Environment.GetEnvironmentVariable("PERF_DATA");
    if (!string.IsNullOrEmpty(path))
        return JsonSerializer.Deserialize<List<User>>(File.ReadAllText(path), new JsonSerializerOptions { Converters = { new BytesFromArray() } })
            ?? throw new InvalidOperationException(path);
    var created = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    return [.. Enumerable.Range(1, rows).Select(id => new User
    {
        id = id,
        username = $"user{id}",
        password = "hash",
        is_enabled = true,
        created = created,
        updated = [1, 2, 3]
    })];
}

async Task Each<T>(List<T> items, Func<List<T>, Task> write)
{
    if (!batched) { await write(items); return; }
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

sealed class BytesFromArray : JsonConverter<byte[]>
{
    public override byte[] Read(ref Utf8JsonReader reader, Type type, JsonSerializerOptions options)
    {
        if (reader.TokenType != JsonTokenType.StartArray) return reader.GetBytesFromBase64();
        var bytes = new List<byte>();
        while (reader.Read() && reader.TokenType != JsonTokenType.EndArray)
            bytes.Add(reader.GetByte());
        return [.. bytes];
    }

    public override void Write(Utf8JsonWriter writer, byte[] value, JsonSerializerOptions options) =>
        writer.WriteBase64StringValue(value);
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
