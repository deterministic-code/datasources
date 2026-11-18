using System.Diagnostics;
using System.Text.Json;
using Backend.Types.View;
using Datasources;

var processClock = Stopwatch.StartNew();
var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var sourceUrl = Environment.GetEnvironmentVariable("DATASOURCE_URL") ?? "sqlite:///./app.sqlite";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "25000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var readIters = int.Parse(Environment.GetEnvironmentVariable("PERF_READ_ITERS") ?? "3");
var batched = provider == "sqlite";
_ = Datasource.Default;
var startupMs = processClock.Elapsed.TotalMilliseconds;
var results = await Task.WhenAll(
    PerformanceTestDefaultRead(),
    PerformanceTestDictionaryRead(),
    PerformanceTestPropertyMappedRead());
Console.WriteLine(JsonSerializer.Serialize(new
{
    sample = "mapping_comparison",
    language = "csharp",
    provider,
    rows,
    chunk = Math.Min(chunk, rows),
    startupMs,
    tests = new { defaultRead = results[0], dictionaryRead = results[1], propertyMappedRead = results[2] },
}, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase }));

async Task<object> PerformanceTestDefaultRead()
{
    var url = await Isolate("default_read");
    await using var ds = DatasourceFactory.Create(new Dictionary<string, string?>
    {
        ["PROVIDER"] = provider,
        ["DATASOURCE_URL"] = url,
    });
    var connection = await ds.CreateConnection();
    await connection.Open();
    double seedMs = 0;
    try
    {
        var addressesBulk = batched
            ? new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new BatchCommandIBulkWriteStrategy<Address>())
            : new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new StoredProcedureBulkWriteStrategy<Address>("bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses"));
        var emailsBulk = batched
            ? new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new BatchCommandIBulkWriteStrategy<Email>())
            : new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new StoredProcedureBulkWriteStrategy<Email>("bulk_create_emails", "bulk_update_emails", "bulk_delete_emails"));
        var contactsBulk = batched
            ? new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new BatchCommandIBulkWriteStrategy<Contact>())
            : new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new StoredProcedureBulkWriteStrategy<Contact>("bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts"));
        contactsBulk.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addressesBulk);
        contactsBulk.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), emailsBulk);
        async Task Setup()
        {
            var seeded = SeedContacts(rows);
            var clock = Stopwatch.StartNew();
            for (var at = 0; at < seeded.Count; at += chunk)
                await contactsBulk.Create(seeded.GetRange(at, Math.Min(chunk, seeded.Count - at)));
            seedMs = clock.Elapsed.TotalMilliseconds;
            await CountContacts(connection);
        }
        async Task<(double ms, double perSec, object counts)> PerformanceDefaultRead()
        {
            var repo = ds.CreateRepository<Contact>("Contacts");
            var loaded = await Collect(repo.Read());
            Check(loaded, "default");
            var times = new List<double>();
            for (var iter = 0; iter < readIters; iter++)
            {
                var clock = Stopwatch.StartNew();
                loaded = await Collect(repo.Read());
                Check(loaded, "default");
                times.Add(clock.Elapsed.TotalMilliseconds);
            }
            times.Sort();
            var ms = times.Count % 2 == 1 ? times[times.Count / 2] : (times[times.Count / 2 - 1] + times[times.Count / 2]) / 2;
            return (ms, rows / (ms / 1000), new { contacts = loaded.Count, addresses = loaded.Sum(row => row.Addresses.Count), emails = loaded.Sum(row => row.Emails.Count) });
        }
        async Task Teardown()
        {
            await connection.Close();
            await DropIsolated("default_read");
        }
        await Setup();
        var read = await PerformanceDefaultRead();
        await Teardown();
        return new { seedMs, readMs = read.ms, readPerSec = read.perSec, read.counts };
    }
    finally
    {
        if (connection.IsOpen) await connection.Close();
    }
}

async Task<object> PerformanceTestDictionaryRead()
{
    var url = await Isolate("dictionary_read");
    await using var ds = DatasourceFactory.Create(new Dictionary<string, string?>
    {
        ["PROVIDER"] = provider,
        ["DATASOURCE_URL"] = url,
    });
    var connection = await ds.CreateConnection();
    await connection.Open();
    double seedMs = 0;
    try
    {
        var snake = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        string[] contactColumns =
        [
            "Id", "Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation",
            "FamilyName", "NickName", "JobTitle", "Organization", "IsStarred", "IsFavorite", "IsDeleted", "GenderTypeId",
        ];
        string[] addressColumns =
        [
            "Id", "Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3",
            "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress",
            "Latitude", "Longitude", "IsPrimary",
        ];
        string[] emailColumns =
        [
            "Id", "Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary",
        ];
        DictionaryMapper<string> Props(IEnumerable<string> names) =>
            new(names.ToDictionary(name => name, name => snake.Map(name)));
        var contactSchema = new Schema(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["Name"] = new("string", false, false),
            ["GivenName"] = new("string", false, false),
            ["AdditionalName"] = new("string", false, true),
            ["Pronunciation"] = new("string", false, true),
            ["FamilyName"] = new("string", false, true),
            ["NickName"] = new("string", false, true),
            ["JobTitle"] = new("string", false, true),
            ["Organization"] = new("string", false, true),
            ["IsStarred"] = new("boolean", false, false),
            ["IsFavorite"] = new("boolean", false, false),
            ["IsDeleted"] = new("boolean", false, false),
            ["GenderTypeId"] = new("integer", false, false),
        });
        var addressSchema = new Schema(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["ContactId"] = new("integer", false, false),
            ["DisplayName"] = new("string", false, true),
            ["Line1"] = new("string", false, false),
            ["Line2"] = new("string", false, true),
            ["Line3"] = new("string", false, true),
            ["Neighborhood"] = new("string", false, true),
            ["District"] = new("string", false, true),
            ["City"] = new("string", false, false),
            ["StateProvince"] = new("string", false, true),
            ["PostalCode"] = new("string", false, true),
            ["CountryCode"] = new("string", false, false),
            ["FormattedAddress"] = new("string", false, true),
            ["Latitude"] = new("decimal", true, false),
            ["Longitude"] = new("decimal", true, false),
            ["IsPrimary"] = new("boolean", false, true),
        });
        var emailSchema = new Schema(new Dictionary<string, SchemaColumn>
        {
            ["Id"] = new("integer", false, false, true),
            ["Uuid"] = new("uuid", false, true),
            ["Created"] = new("datetime", false, true),
            ["Updated"] = new("datetime", false, true),
            ["Version"] = new("binary", false, true),
            ["EmailTypeId"] = new("integer", false, false),
            ["ContactId"] = new("integer", false, false),
            ["Address"] = new("string", false, false),
            ["DisplayName"] = new("string", false, true),
            ["IsPrimary"] = new("boolean", false, false),
        });
        var addressesBulk = batched
            ? new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new BatchCommandIBulkWriteStrategy<Address>())
            : new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new StoredProcedureBulkWriteStrategy<Address>("bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses"));
        var emailsBulk = batched
            ? new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new BatchCommandIBulkWriteStrategy<Email>())
            : new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new StoredProcedureBulkWriteStrategy<Email>("bulk_create_emails", "bulk_update_emails", "bulk_delete_emails"));
        var contactsBulk = batched
            ? new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new BatchCommandIBulkWriteStrategy<Contact>())
            : new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new StoredProcedureBulkWriteStrategy<Contact>("bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts"));
        contactsBulk.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addressesBulk);
        contactsBulk.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), emailsBulk);
        var addresses = new ReadonlyRepository<Address>(connection, new()
        {
            Entity = "Address",
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "addresses" }),
            PropertyMappings = Props(addressColumns),
            ReadProperties = addressColumns,
            Schema = addressSchema,
        });
        var emails = new ReadonlyRepository<Email>(connection, new()
        {
            Entity = "Email",
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Email"] = "emails" }),
            PropertyMappings = Props(emailColumns),
            ReadProperties = emailColumns,
            Schema = emailSchema,
        });
        var contacts = new ReadonlyRepository<Contact>(connection, new()
        {
            Entity = "Contact",
            EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
            PropertyMappings = Props(contactColumns),
            ReadProperties = contactColumns,
            Schema = contactSchema,
        });
        contacts.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addresses);
        contacts.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), emails);
        async Task Setup()
        {
            var seeded = SeedContacts(rows);
            var clock = Stopwatch.StartNew();
            for (var at = 0; at < seeded.Count; at += chunk)
                await contactsBulk.Create(seeded.GetRange(at, Math.Min(chunk, seeded.Count - at)));
            seedMs = clock.Elapsed.TotalMilliseconds;
            await CountContacts(connection);
        }
        async Task<(double ms, double perSec, object counts)> PerformanceDictionaryRead()
        {
            var loaded = await Collect(contacts.Read());
            Check(loaded, "dictionary");
            var times = new List<double>();
            for (var iter = 0; iter < readIters; iter++)
            {
                var clock = Stopwatch.StartNew();
                loaded = await Collect(contacts.Read());
                Check(loaded, "dictionary");
                times.Add(clock.Elapsed.TotalMilliseconds);
            }
            times.Sort();
            var ms = times.Count % 2 == 1 ? times[times.Count / 2] : (times[times.Count / 2 - 1] + times[times.Count / 2]) / 2;
            return (ms, rows / (ms / 1000), new { contacts = loaded.Count, addresses = loaded.Sum(row => row.Addresses.Count), emails = loaded.Sum(row => row.Emails.Count) });
        }
        async Task Teardown()
        {
            await connection.Close();
            await DropIsolated("dictionary_read");
        }
        await Setup();
        var read = await PerformanceDictionaryRead();
        await Teardown();
        return new { seedMs, readMs = read.ms, readPerSec = read.perSec, read.counts };
    }
    finally
    {
        if (connection.IsOpen) await connection.Close();
    }
}

async Task<object> PerformanceTestPropertyMappedRead()
{
    var url = await Isolate("property_mapped_read");
    await using var ds = DatasourceFactory.Create(new Dictionary<string, string?>
    {
        ["PROVIDER"] = provider,
        ["DATASOURCE_URL"] = url,
    });
    var connection = await ds.CreateConnection();
    await connection.Open();
    double seedMs = 0;
    try
    {
        var addressesBulk = batched
            ? new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new BatchCommandIBulkWriteStrategy<Address>())
            : new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(), new StoredProcedureBulkWriteStrategy<Address>("bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses"));
        var emailsBulk = batched
            ? new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new BatchCommandIBulkWriteStrategy<Email>())
            : new BulkCRUDRepository<Email>(connection, EmailsRepository.Options(), new StoredProcedureBulkWriteStrategy<Email>("bulk_create_emails", "bulk_update_emails", "bulk_delete_emails"));
        var contactsBulk = batched
            ? new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new BatchCommandIBulkWriteStrategy<Contact>())
            : new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(), new StoredProcedureBulkWriteStrategy<Contact>("bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts"));
        contactsBulk.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addressesBulk);
        contactsBulk.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), emailsBulk);
        async Task Setup()
        {
            var seeded = SeedContacts(rows);
            var clock = Stopwatch.StartNew();
            for (var at = 0; at < seeded.Count; at += chunk)
                await contactsBulk.Create(seeded.GetRange(at, Math.Min(chunk, seeded.Count - at)));
            seedMs = clock.Elapsed.TotalMilliseconds;
            await CountContacts(connection);
        }
        async Task<(double ms, double perSec, object counts)> PerformancePropertyMappedRead()
        {
            var repo = ContactsRepository.CreateReadonly(connection);
            var loaded = await Collect(repo.Read());
            Check(loaded, "propertyMapped");
            var times = new List<double>();
            for (var iter = 0; iter < readIters; iter++)
            {
                var clock = Stopwatch.StartNew();
                loaded = await Collect(repo.Read());
                Check(loaded, "propertyMapped");
                times.Add(clock.Elapsed.TotalMilliseconds);
            }
            times.Sort();
            var ms = times.Count % 2 == 1 ? times[times.Count / 2] : (times[times.Count / 2 - 1] + times[times.Count / 2]) / 2;
            return (ms, rows / (ms / 1000), new { contacts = loaded.Count, addresses = loaded.Sum(row => row.Addresses.Count), emails = loaded.Sum(row => row.Emails.Count) });
        }
        async Task Teardown()
        {
            await connection.Close();
            await DropIsolated("property_mapped_read");
        }
        await Setup();
        var read = await PerformancePropertyMappedRead();
        await Teardown();
        return new { seedMs, readMs = read.ms, readPerSec = read.perSec, read.counts };
    }
    finally
    {
        if (connection.IsOpen) await connection.Close();
    }
}

async Task<string> Isolate(string name)
{
    var catalog = $"mapping_{name}";
    var url = WithDatasource(sourceUrl, catalog);
    if (provider == "sqlite")
    {
        var file = DatasourceUrlParser.Parse(url)?.Datasource;
        if (file is not null)
        {
            TryDelete(file);
            TryDelete(file + "-wal");
            TryDelete(file + "-shm");
        }
        MigrateUp(url);
        return url;
    }
    await using var admin = Admin();
    var connection = await admin.CreateConnection();
    await connection.Open();
    try
    {
        await Exec(connection, provider == "postgres"
            ? $"DROP DATABASE IF EXISTS {catalog} WITH (FORCE)"
            : $"DROP DATABASE IF EXISTS `{catalog}`");
        await Exec(connection, provider == "postgres"
            ? $"CREATE DATABASE {catalog}"
            : $"CREATE DATABASE `{catalog}`");
    }
    finally
    {
        await connection.Close();
    }
    MigrateUp(url);
    return url;
}

void MigrateUp(string url)
{
    Npx("migrate-setup", "--provider", provider, "--connection", url);
    Npx("migrate-up", "--provider", provider, "--connection", url, "--migrations-root", "sql");
}

void Npx(params string[] args)
{
    var start = new ProcessStartInfo("npx") { RedirectStandardOutput = true, RedirectStandardError = true };
    foreach (var arg in args) start.ArgumentList.Add(arg);
    using var process = Process.Start(start) ?? throw new InvalidOperationException("npx");
    process.WaitForExit();
    if (process.ExitCode != 0)
        throw new InvalidOperationException(process.StandardError.ReadToEnd() + process.StandardOutput.ReadToEnd());
}

async Task DropIsolated(string name)
{
    var catalog = $"mapping_{name}";
    if (provider == "sqlite")
    {
        var file = DatasourceUrlParser.Parse(WithDatasource(sourceUrl, catalog))?.Datasource;
        if (file is null) return;
        TryDelete(file);
        TryDelete(file + "-wal");
        TryDelete(file + "-shm");
        return;
    }
    await using var admin = Admin();
    var connection = await admin.CreateConnection();
    await connection.Open();
    try
    {
        await Exec(connection, provider == "postgres"
            ? $"DROP DATABASE IF EXISTS {catalog} WITH (FORCE)"
            : $"DROP DATABASE IF EXISTS `{catalog}`");
    }
    finally
    {
        await connection.Close();
    }
}

IDatasource Admin() =>
    DatasourceFactory.Create(new Dictionary<string, string?>
    {
        ["PROVIDER"] = provider,
        ["DATASOURCE_URL"] = provider == "postgres" ? WithDatasource(sourceUrl, "postgres") : sourceUrl,
    });

static string WithDatasource(string url, string name)
{
    var parsed = DatasourceUrlParser.Parse(url);
    if (parsed?.Provider == "sqlite")
    {
        var file = parsed.Datasource ?? "app.sqlite";
        var dir = Path.GetDirectoryName(file);
        var dest = Path.Combine(string.IsNullOrEmpty(dir) ? "." : dir, $"{name}.sqlite");
        return $"sqlite:///{dest.Replace('\\', '/')}";
    }
    var slash = url.LastIndexOf('/');
    var query = url.IndexOf('?', slash < 0 ? 0 : slash);
    if (slash < 0) return url;
    return query < 0 ? $"{url[..(slash + 1)]}{name}" : $"{url[..(slash + 1)]}{name}{url[query..]}";
}

static async Task Exec(IConnection connection, string sql) =>
    await connection.CreateCommand().Execute(Native.N(sql));

async Task<object> CountContacts(IConnection connection)
{
    var reader = await connection.CreateCommand().ExecuteReader(Native.N("SELECT COUNT(*) AS n FROM contacts"));
    try
    {
        await reader.Next();
        var contacts = Convert.ToInt32(reader.Current!["n"].ToObject());
        if (contacts != rows) throw new InvalidOperationException($"create count contacts={contacts}");
        return new { contacts };
    }
    finally
    {
        await reader.DisposeAsync();
    }
}

static void TryDelete(string path)
{
    try { File.Delete(path); } catch (IOException) { } catch (UnauthorizedAccessException) { }
}

static async Task<List<T>> Collect<T>(IAsyncEnumerable<T> rows)
{
    var list = new List<T>();
    await foreach (var row in rows) list.Add(row);
    return list;
}

void Check(List<Contact> loaded, string label)
{
    if (loaded.Count != rows) throw new InvalidOperationException($"{label} count {loaded.Count}");
    foreach (var row in loaded)
    {
        if (row.Addresses.Count != 2 || row.Emails.Count != 2)
            throw new InvalidOperationException($"{label} related {row.GivenName}");
        if (row.Uuid == Guid.Empty || row.Addresses.Any(item => item.Uuid == Guid.Empty) || row.Emails.Any(item => item.Uuid == Guid.Empty))
            throw new InvalidOperationException($"{label} uuid {row.GivenName}");
        if (!Mapped(row.Created) || !Mapped(row.Updated)
            || row.Addresses.Any(item => !Mapped(item.Created) || !Mapped(item.Updated))
            || row.Emails.Any(item => !Mapped(item.Created) || !Mapped(item.Updated)))
            throw new InvalidOperationException($"{label} datetime {row.GivenName}");
        if (!row.GivenName.StartsWith("Given") || !int.TryParse(row.GivenName["Given".Length..], out var n))
            throw new InvalidOperationException($"{label} givenName {row.GivenName}");
        if (row.IsStarred != (n % 2 == 0) || row.IsFavorite != (n % 3 == 0) || row.IsDeleted != (n % 5 == 0))
            throw new InvalidOperationException($"{label} boolean {row.GivenName}");
        if (!row.Addresses[0].IsPrimary || row.Addresses[1].IsPrimary || !row.Emails[0].IsPrimary || row.Emails[1].IsPrimary)
            throw new InvalidOperationException($"{label} isPrimary {row.GivenName}");
    }
}

static bool Mapped(DateTime value) => value != default && value.Year >= 2020;

static List<Contact> SeedContacts(int total)
{
    var contacts = new List<Contact>(total);
    for (var i = 1; i <= total; i++)
        contacts.Add(new Contact
        {
            Name = $"Contact {i}",
            GivenName = $"Given{i}",
            FamilyName = $"Family{i}",
            GenderTypeId = 1 + i % 3,
            IsStarred = i % 2 == 0,
            IsFavorite = i % 3 == 0,
            IsDeleted = i % 5 == 0,
            Addresses =
            [
                new Address { Line1 = $"{i} Line 1", City = $"City{i % 50}", CountryCode = "US", IsPrimary = true },
                new Address { Line1 = $"{i} Line 2", City = $"City{(i + 1) % 50}", CountryCode = "US", IsPrimary = false },
            ],
            Emails =
            [
                new Email { Address = $"c{i}.e1@example.com", DisplayName = "Email 1", EmailTypeId = 1, IsPrimary = true },
                new Email { Address = $"c{i}.e2@example.com", DisplayName = "Email 2", EmailTypeId = 2, IsPrimary = false },
            ],
        });
    return contacts;
}
