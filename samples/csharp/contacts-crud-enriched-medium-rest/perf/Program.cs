using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;
using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var strategyName = Environment.GetEnvironmentVariable("PERF_STRATEGY") ?? "batch";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "1000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var url = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? $"sqlite:///{Path.Combine(Path.GetTempPath(), "contacts-perf.sqlite")}";
var reading = (Environment.GetEnvironmentVariable("PERF_PHASE") ?? "write") == "read";
var batched = strategyName == "batch" || strategyName == "procedure" && provider == "sqlite";
MigrateUp(provider, url);
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
    var counts = new
    {
        contacts = 0,
        addresses = 0,
        phones = 0,
        settings = 0
    };
    var createMs = 0d;
    var updateMs = 0d;
    var relatedCount = int.Parse(Environment.GetEnvironmentVariable("PERF_RELATED") ?? "10");
    var total = rows;
    if (!reading)
    {
        var contacts = LoadContacts();
        createMs = await Time(() => Each(contacts, batch => contactsRepo.Create(batch)));
        counts = new
        {
            contacts = await Count(connection, "contacts"),
            addresses = await Count(connection, "addresses"),
            phones = await Count(connection, "phones"),
            settings = await Count(connection, "contact_settings")
        };
        var each = (
            addresses: contacts.FirstOrDefault()?.addresses.Count ?? 0,
            phones: contacts.FirstOrDefault()?.phones.Count ?? 0,
            settings: contacts.FirstOrDefault()?.settings.Count ?? 0
        );
        if (counts.contacts != contacts.Count || counts.addresses != contacts.Count * each.addresses || counts.phones != contacts.Count * each.phones || counts.settings != contacts.Count * each.settings)
            throw new InvalidOperationException($"create count {counts}");
        foreach (var contact in contacts)
        {
            contact.notes = "updated";
            foreach (var address in contact.addresses) address.city = "City";
            foreach (var phone in contact.phones) phone.label = "w";
            foreach (var key in contact.settings.Keys.ToList()) contact.settings[key] = "dark";
        }
        updateMs = await Time(() => Each(contacts, batch => contactsRepo.Update(batch)));
        total = contacts.Count;
        relatedCount = each.addresses;
    }
    else
    {
        counts = new
        {
            contacts = await Count(connection, "contacts"),
            addresses = await Count(connection, "addresses"),
            phones = await Count(connection, "phones"),
            settings = await Count(connection, "contact_settings")
        };
        total = counts.contacts;
        if (total == 0) throw new InvalidOperationException("read store is empty; run npm run test:perf first");
    }
    var hashsetRepo = Repo<Contact>(connection, new()
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
    var hashset = new HashsetOptimizedReadonlyEnrichmentStrategy<Contact>();
    hashsetRepo.AddEnriched(new("addresses", "id", "contactId", Type: EnrichmentPropertyType.List), addressesRepo, hashset);
    hashsetRepo.AddEnriched(new("phones", "id", "contactId", Type: EnrichmentPropertyType.List), phonesRepo, hashset);
    hashsetRepo.AddEnriched(new("settings", "id", "contactId", "value", EnrichmentPropertyType.Dictionary), settingsRepo, hashset);
    var joinRepo = new CRUDRepository<Row>(connection, new()
    {
        Entity = "Contact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["Contact"] = "contacts",
            ["Address"] = "addresses",
            ["Phone"] = "phones",
            ["ContactSettings"] = "contact_settings"
        }),
        PropertyMappings = new DictionaryMapper<string>(new Dictionary<string, string>
        {
            ["firstName"] = "first_name",
            ["contactId"] = "contact_id"
        })
    });
    object? reads = reading ? await ProfileReads(contactsRepo, hashsetRepo, joinRepo, total, relatedCount) : null;
    Console.WriteLine(JsonSerializer.Serialize(new
    {
        sample = "contacts-crud-enriched-medium-rest",
        language = "csharp",
        provider,
        strategy = strategyName,
        rows = total,
        chunk = batched ? Math.Min(chunk, total) : total,
        createMs,
        updateMs,
        createPerSec = createMs == 0 ? 0 : total / (createMs / 1000),
        updatePerSec = updateMs == 0 ? 0 : total / (updateMs / 1000),
        counts,
        reads
    }));
}
finally
{
    try { await connection.Close(); } catch { }
}

List<Contact> LoadContacts()
{
    var path = Environment.GetEnvironmentVariable("PERF_DATA");
    if (!string.IsNullOrEmpty(path))
        return JsonSerializer.Deserialize<List<Contact>>(File.ReadAllText(path), new JsonSerializerOptions { Converters = { new BytesFromArray() } })
            ?? throw new InvalidOperationException(path);
    var created = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
    var each = int.Parse(Environment.GetEnvironmentVariable("PERF_RELATED") ?? "10");
    int Child(int contactId, int index) => (contactId - 1) * each + index + 1;
    return [.. Enumerable.Range(1, rows).Select(id => new Contact
    {
        id = id,
        uuid = $"00000000-0000-0000-0000-{id:x12}",
        created = created,
        updated = created,
        version = [1],
        contactSourceId = 1,
        firstName = $"First{id}",
        lastName = $"Last{id}",
        email = $"c{id}@ex.test",
        addresses = [.. Enumerable.Range(0, each).Select(n => new Address { id = Child(id, n), uuid = $"a{Child(id, n)}", created = created, updated = created, version = [1], addressTypeId = 1, line1 = "1 Main", city = "Town" })],
        phones = [.. Enumerable.Range(0, each).Select(n => new Phone { id = Child(id, n), uuid = $"p{Child(id, n)}", created = created, updated = created, version = [1], phoneTypeId = 1, number = "5550000", label = "m" })],
        settings = Enumerable.Range(0, each).ToDictionary(n => $"k{n}", _ => "light")
    })];
}

BulkCRUDRepository<T> Repo<T>(IConnection connection, CRUDRepositoryOptions<T> options, string create, string update, string delete)
    where T : new() =>
    strategyName switch
    {
        "single" => new BulkCRUDRepository<T>(connection, options),
        _ when batched => new BulkCRUDRepository<T>(connection, options, new BatchCommandIBulkWriteStrategy<T>()),
        "procedure" => new BulkCRUDRepository<T>(connection, options, new StoredProcedureBulkWriteStrategy<T>(create, update, delete)),
        _ => throw new InvalidOperationException(strategyName)
    };

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

async Task<object> ProfileReads(
    BulkCRUDRepository<Contact> contactsRepo,
    BulkCRUDRepository<Contact> hashsetRepo,
    CRUDRepository<Row> joinRepo,
    int total,
    int relatedCount)
{
    var items = Math.Min(int.Parse(Environment.GetEnvironmentVariable("PERF_READ_ITEMS") ?? "100"), total);
    var iters = int.Parse(Environment.GetEnvironmentVariable("PERF_READ_ITERS") ?? "5");
    var rng = new Random(42);
    var mapper = new FlatToNestedMapper(new TypeMap
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "id",
            ["firstName"] = "firstName",
            ["addresses"] = new TypeMapGroup { Key = "addressId", Fields = new Dictionary<string, TypeMapField> { ["id"] = "addressId" } },
            ["phones"] = new TypeMapGroup { Key = "phoneId", Fields = new Dictionary<string, TypeMapField> { ["id"] = "phoneId" } },
            ["settings"] = new TypeMapGroup { Key = "settingKey", Fields = new Dictionary<string, TypeMapField> { ["key"] = "settingKey", ["value"] = "settingValue" } }
        }
    });
    var times = new Dictionary<string, Dictionary<string, List<double>>>
    {
        ["default"] = Shapes(),
        ["hashset"] = Shapes(),
        ["flat"] = Shapes()
    };
    await RunReads(contactsRepo, hashsetRepo, joinRepo, mapper, Sample(rng, total, items, 0), relatedCount, false);
    for (var iter = 0; iter < iters; iter++)
    {
        var sample = Sample(rng, total, items, iter);
        var measured = await RunReads(contactsRepo, hashsetRepo, joinRepo, mapper, sample, relatedCount, iter == 0);
        foreach (var (mode, shapes) in measured)
            foreach (var (shape, ms) in shapes)
                times[mode][shape].Add(ms);
    }
    return times.ToDictionary(mode => mode.Key, mode => mode.Value.ToDictionary(shape => shape.Key, shape => new
    {
        ms = Median(shape.Value),
        perSec = items / (Median(shape.Value) / 1000)
    }));
}

static Dictionary<string, List<double>> Shapes() => new() { ["ids"] = [], ["where"] = [], ["firstName"] = [] };

static (int[] Ids, int Lo) Sample(Random rng, int total, int items, int iter)
{
    var ids = Enumerable.Range(1, total).ToArray();
    for (var i = 0; i < items; i++)
    {
        var swap = rng.Next(i, total);
        (ids[i], ids[swap]) = (ids[swap], ids[i]);
    }
    return (ids[..items], items >= total ? 0 : iter * items % (total - items + 1));
}

async Task<Dictionary<string, Dictionary<string, double>>> RunReads(
    BulkCRUDRepository<Contact> contactsRepo,
    BulkCRUDRepository<Contact> hashsetRepo,
    CRUDRepository<Row> joinRepo,
    FlatToNestedMapper mapper,
    (int[] Ids, int Lo) sample,
    int relatedCount,
    bool check)
{
    var shapes = new Dictionary<string, IWhere>
    {
        ["ids"] = new WhereText($"id IN ({string.Join(",", sample.Ids)})"),
        ["where"] = new WhereText($"id > {sample.Lo} AND id <= {sample.Lo + sample.Ids.Length}"),
        ["firstName"] = new WhereText($"first_name IN ({string.Join(",", sample.Ids.Select(id => $"'First{id}'"))})")
    };
    var measured = new Dictionary<string, Dictionary<string, double>>
    {
        ["default"] = [],
        ["hashset"] = [],
        ["flat"] = []
    };
    foreach (var (shape, where) in shapes)
    {
        measured["default"][shape] = await Time(async () => CheckEnriched(await Collect(contactsRepo.Read(where)), sample.Ids.Length, relatedCount, check));
        measured["hashset"][shape] = await Time(async () => CheckEnriched(await Collect(hashsetRepo.Read(where)), sample.Ids.Length, relatedCount, check));
        measured["flat"][shape] = await Time(async () => CheckMapped(mapper.Map(await Collect(joinRepo.Read(JoinSelect(JoinWhere(shape, sample))))), sample.Ids.Length, relatedCount, check));
    }
    return measured;
}

static IWhere JoinWhere(string shape, (int[] Ids, int Lo) sample) =>
    shape switch
    {
        "ids" => new WhereText($"contacts.id IN ({string.Join(",", sample.Ids)})"),
        "where" => new WhereText($"contacts.id > {sample.Lo} AND contacts.id <= {sample.Lo + sample.Ids.Length}"),
        "firstName" => new WhereText($"contacts.first_name IN ({string.Join(",", sample.Ids.Select(id => $"'First{id}'"))})"),
        _ => throw new InvalidOperationException(shape)
    };

static ISelectStatement JoinSelect(IWhere where) =>
    new SelectStatement(new()
    {
        Table = "Contact",
        Fields =
        [
            "Contact.id AS id",
            "Contact.firstName AS firstName",
            "Address.id AS addressId",
            "Phone.id AS phoneId",
            "ContactSettings.key AS settingKey",
            "ContactSettings.value AS settingValue"
        ],
        Joins =
        [
            Join.Left("Address", "Contact.id", "Address.contactId"),
            Join.Left("Phone", "Contact.id", "Phone.contactId"),
            Join.Left("ContactSettings", "Contact.id", "ContactSettings.contactId")
        ],
        Where = where
    });

static void CheckEnriched(List<Contact> loaded, int expected, int relatedCount, bool check)
{
    if (!check) return;
    if (loaded.Count != expected) throw new InvalidOperationException($"read count {loaded.Count}");
    if (loaded.Any(contact => contact.addresses.Count != relatedCount || contact.phones.Count != relatedCount || contact.settings.Count != relatedCount))
        throw new InvalidOperationException("read related");
}

static void CheckMapped(IReadOnlyList<Dictionary<string, MappedValue>> loaded, int expected, int relatedCount, bool check)
{
    if (!check) return;
    if (loaded.Count != expected) throw new InvalidOperationException($"flat count {loaded.Count}");
    if (loaded.Any(row => CountMany(row, "addresses") != relatedCount || CountMany(row, "phones") != relatedCount || CountMany(row, "settings") != relatedCount))
        throw new InvalidOperationException("flat related");
}

static int CountMany(Dictionary<string, MappedValue> row, string name) =>
    row.TryGetValue(name, out var value) && value is MappedValue.Many many ? many.Items.Count : 0;

static async Task<List<T>> Collect<T>(IAsyncEnumerable<T> items)
{
    var list = new List<T>();
    await foreach (var item in items) list.Add(item);
    return list;
}

static double Median(List<double> values)
{
    var sorted = values.OrderBy(value => value).ToArray();
    var mid = sorted.Length / 2;
    return sorted.Length % 2 == 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

static void MigrateUp(string provider, string url)
{
    var repo = RepoRoot();
    var sql = Path.Combine(Path.GetTempPath(), $"contacts-perf-sql-{Guid.NewGuid():N}");
    Directory.CreateDirectory(sql);
    var create = Exec("node", [
        "--import", "tsx", Path.Combine(repo, "cli", "src", "main.ts"),
        "migrations", "create",
        "--input", Path.Combine(repo, "samples", "contacts-crud-enriched-medium-rest-shared", "deterministic"),
        "--migrations-root", Path.Combine(sql, "sql"),
    ], Path.Combine(repo, "cli"));
    if (create.Exit != 0) throw new InvalidOperationException($"{create.Stderr}\n{create.Stdout}");
    var up = Exec("node", [
        Path.Combine(repo, "scripts", "migraters.mjs"),
        "up", "--provider", provider, "--connection", url, "--migrations-root", Path.Combine(sql, "sql"),
    ], repo);
    if (up.Exit != 0) throw new InvalidOperationException($"{up.Stderr}\n{up.Stdout}");
}

static (int Exit, string Stdout, string Stderr) Exec(string file, IEnumerable<string> args, string cwd)
{
    var start = new ProcessStartInfo(file) { WorkingDirectory = cwd, RedirectStandardOutput = true, RedirectStandardError = true };
    foreach (var arg in args) start.ArgumentList.Add(arg);
    var process = Process.Start(start) ?? throw new InvalidOperationException(file);
    var output = process.StandardOutput.ReadToEndAsync();
    var error = process.StandardError.ReadToEndAsync();
    process.WaitForExit();
    return (process.ExitCode, output.Result, error.Result);
}

static string RepoRoot()
{
    for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        if (File.Exists(Path.Combine(dir.FullName, "package.json")) && File.ReadAllText(Path.Combine(dir.FullName, "package.json")).Contains("\"setup\""))
            return dir.FullName;
    throw new InvalidOperationException("repo");
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
