using System.Diagnostics;
using System.Text.Json;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var strategyName = Environment.GetEnvironmentVariable("PERF_STRATEGY") ?? "batch";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "1000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? "sqlite:///:memory:";
var shared = Shared("contacts-crud-enriched-medium-rest-shared");
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
    foreach (var sql in File.ReadAllText(Path.Combine(shared, $"schema-{provider}.sql")).Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
        await command.Execute(Native.N(sql));
    if (provider is "mysql" or "postgres")
        foreach (var sql in File.ReadAllText(Path.Combine(shared, $"bulk-{provider}.sql")).Split("---", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(Native.N(sql));
    var now = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    var stamp = provider == "mysql" ? "2024-01-01 00:00:00" : now.ToString("o");
    var blob = provider == "mysql" ? "0x01" : provider == "postgres" ? "'\\x01'" : "x'01'";
    foreach (var table in new[] { "contact_sources", "address_types", "phone_types" })
        await command.Execute(Native.N($"INSERT INTO {table} (id, uuid, created, updated, version, name) VALUES (1, '00000000-0000-0000-0000-000000000001', '{stamp}', '{stamp}', {blob}, 'seed')"));
    var settingsRepo = Repo<Setting>(connection, new()
    {
        Entity = "ContactSettings",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["ContactSettings"] = "contact_settings" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string> { ["contactId"] = "contact_id" }),
        Ids = ["contactId", "key"]
    }, "bulk_create_setting", "bulk_update_setting", "bulk_delete_setting");
    var addressesRepo = Repo<Address>(connection, new()
    {
        Entity = "Address",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "addresses" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string> { ["contactId"] = "contact_id", ["addressTypeId"] = "address_type_id" })
    }, "bulk_create_address", "bulk_update_address", "bulk_delete_address");
    var phonesRepo = Repo<Phone>(connection, new()
    {
        Entity = "Phone",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Phone"] = "phones" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string> { ["contactId"] = "contact_id", ["phoneTypeId"] = "phone_type_id" })
    }, "bulk_create_phone", "bulk_update_phone", "bulk_delete_phone");
    var contactsRepo = Repo<Contact>(connection, new()
    {
        Entity = "Contact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["contactSourceId"] = "contact_source_id",
            ["firstName"] = "first_name",
            ["lastName"] = "last_name"
        })
    }, "bulk_create_contact", "bulk_update_contact", "bulk_delete_contact");
    contactsRepo.AddEnriched(new("addresses", "id", "contactId", Type: EnrichmentPropertyType.List), addressesRepo);
    contactsRepo.AddEnriched(new("phones", "id", "contactId", Type: EnrichmentPropertyType.List), phonesRepo);
    contactsRepo.AddEnriched(new("settings", "id", "contactId", "value", EnrichmentPropertyType.Dictionary), settingsRepo);
    var contacts = Enumerable.Range(1, rows).Select(id => new Contact
    {
        id = strategyName == "batch" ? id : 0,
        uuid = $"00000000-0000-0000-0000-{id:x12}",
        created = now,
        updated = now,
        version = [1],
        contactSourceId = 1,
        firstName = $"First{id}",
        lastName = $"Last{id}",
        email = $"c{id}@ex.test",
        addresses = [new Address { id = strategyName == "batch" ? id : 0, uuid = $"a{id}", created = now, updated = now, version = [1], addressTypeId = 1, line1 = "1 Main", city = "Town" }],
        phones = [new Phone { id = strategyName == "batch" ? id : 0, uuid = $"p{id}", created = now, updated = now, version = [1], phoneTypeId = 1, number = "5550000", label = "m" }],
        settings = new Dictionary<string, string> { ["theme"] = "light" }
    }).ToList();
    var createMs = await Time(() => Each(contacts, batch => contactsRepo.Create(batch)));
    var counts = new
    {
        contacts = await Count(connection, "contacts"),
        addresses = await Count(connection, "addresses"),
        phones = await Count(connection, "phones"),
        settings = await Count(connection, "contact_settings")
    };
    if (counts.contacts != rows || counts.addresses != rows || counts.phones != rows || counts.settings != rows)
        throw new InvalidOperationException($"create count {counts}");
    foreach (var contact in contacts)
    {
        contact.notes = "updated";
        contact.addresses[0].city = "City";
        contact.phones[0].label = "w";
        contact.settings = new Dictionary<string, string> { ["theme"] = "dark" };
    }
    var updateMs = await Time(() => Each(contacts, batch => contactsRepo.Update(batch)));
    Console.WriteLine(JsonSerializer.Serialize(new
    {
        sample = "contacts-crud-enriched-medium-rest",
        language = "csharp",
        provider,
        strategy = strategyName,
        rows,
        chunk = strategyName == "batch" ? chunk : rows,
        createMs,
        updateMs,
        createPerSec = rows / (createMs / 1000),
        updatePerSec = rows / (updateMs / 1000),
        counts
    }));
}
finally
{
    try { await connection.Close(); } catch { }
}

BulkCRUDRepository<T> Repo<T>(IConnection connection, CRUDRepositoryOptions<T> options, string create, string update, string delete)
    where T : new() =>
    strategyName switch
    {
        "single" => new BulkCRUDRepository<T>(connection, options),
        "batch" => new BulkCRUDRepository<T>(connection, options, new BatchCommandIBulkWriteStrategy<T>()),
        "procedure" => new BulkCRUDRepository<T>(connection, options, new StoredProcedureBulkWriteStrategy<T>(create, update, delete)),
        _ => throw new InvalidOperationException(strategyName)
    };

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

sealed class Contact
{
    public int id { get; set; }
    public string uuid { get; set; } = "";
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    public byte[] version { get; set; } = [];
    public int contactSourceId { get; set; }
    public string firstName { get; set; } = "";
    public string lastName { get; set; } = "";
    public string email { get; set; } = "";
    public string notes { get; set; } = "";
    public List<Address> addresses { get; set; } = [];
    public List<Phone> phones { get; set; } = [];
    public Dictionary<string, string> settings { get; set; } = [];
}

sealed class Address
{
    public int id { get; set; }
    public string uuid { get; set; } = "";
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    public byte[] version { get; set; } = [];
    public int contactId { get; set; }
    public int addressTypeId { get; set; }
    public string line1 { get; set; } = "";
    public string city { get; set; } = "";
}

sealed class Phone
{
    public int id { get; set; }
    public string uuid { get; set; } = "";
    public DateTime created { get; set; }
    public DateTime updated { get; set; }
    public byte[] version { get; set; } = [];
    public int contactId { get; set; }
    public int phoneTypeId { get; set; }
    public string number { get; set; } = "";
    public string label { get; set; } = "";
}

sealed class Setting
{
    public int contactId { get; set; }
    public string key { get; set; } = "";
    public string value { get; set; } = "";
}
