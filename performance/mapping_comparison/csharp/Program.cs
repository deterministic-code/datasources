using System.Diagnostics;
using System.Text.Json;
using Backend.Types.View;
using Datasources;

var provider = Environment.GetEnvironmentVariable("PROVIDER") ?? "sqlite";
var rows = int.Parse(Environment.GetEnvironmentVariable("PERF_ROWS") ?? "25000");
var chunk = int.Parse(Environment.GetEnvironmentVariable("PERF_CHUNK") ?? "500");
var readIters = int.Parse(Environment.GetEnvironmentVariable("PERF_READ_ITERS") ?? "3");
var batched = provider == "sqlite";
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

await using var ds = Datasource.Default;
var connection = await ds.CreateConnection();
await connection.Open();
try
{
    var sparseContact = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["GivenName"] = "given_name",
        ["AdditionalName"] = "additional_name",
        ["FamilyName"] = "family_name",
        ["NickName"] = "nick_name",
        ["JobTitle"] = "job_title",
        ["IsStarred"] = "is_starred",
        ["IsFavorite"] = "is_favorite",
        ["IsDeleted"] = "is_deleted",
        ["GenderTypeId"] = "gender_type_id",
    });
    var sparseAddress = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["ContactId"] = "contact_id",
        ["DisplayName"] = "display_name",
        ["Line1"] = "line_1",
        ["Line2"] = "line_2",
        ["Line3"] = "line_3",
        ["StateProvince"] = "state_province",
        ["PostalCode"] = "postal_code",
        ["CountryCode"] = "country_code",
        ["FormattedAddress"] = "formatted_address",
        ["IsPrimary"] = "is_primary",
    });
    var sparseEmail = new DictionaryMapper<string>(new Dictionary<string, string>
    {
        ["ContactId"] = "contact_id",
        ["DisplayName"] = "display_name",
        ["IsPrimary"] = "is_primary",
        ["EmailTypeId"] = "email_type_id",
    });

    var addressesBulk = Repo<Address>(connection, new()
    {
        Entity = "Address",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "addresses" }),
        PropertyMappings = sparseAddress,
        Ids = ["Id"],
        CreateProperties = addressColumns,
        UpdateProperties = addressColumns,
        ReadProperties = addressColumns,
        Schema = addressSchema,
    }, "bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses");
    var emailsBulk = Repo<Email>(connection, new()
    {
        Entity = "Email",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Email"] = "emails" }),
        PropertyMappings = sparseEmail,
        Ids = ["Id"],
        CreateProperties = emailColumns,
        UpdateProperties = emailColumns,
        ReadProperties = emailColumns,
        Schema = emailSchema,
    }, "bulk_create_emails", "bulk_update_emails", "bulk_delete_emails");
    var contactsBulk = Repo<Contact>(connection, new()
    {
        Entity = "Contact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
        PropertyMappings = sparseContact,
        Ids = ["Id"],
        CreateProperties = contactColumns,
        UpdateProperties = contactColumns,
        ReadProperties = contactColumns,
        Schema = contactSchema,
    }, "bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts");
    contactsBulk.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addressesBulk);
    contactsBulk.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), emailsBulk);

    var seeded = SeedContacts(rows);
    var createMs = await Time(() => Each(seeded, batch => contactsBulk.Create(batch)));
    var counts = new
    {
        contacts = await Count(connection, "contacts"),
        addresses = await Count(connection, "addresses"),
        emails = await Count(connection, "emails"),
    };
    var expectedAddresses = seeded.Sum(row => row.Addresses.Count);
    var expectedEmails = seeded.Sum(row => row.Emails.Count);
    if (counts.contacts != rows || counts.addresses != expectedAddresses || counts.emails != expectedEmails)
        throw new InvalidOperationException($"create count {JsonSerializer.Serialize(counts)}");

    var defaultRepo = ds.CreateRepository<Contact>("Contacts");
    var dictionaryAddresses = new ReadonlyRepository<Address>(connection, new()
    {
        Entity = "Address",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Address"] = "addresses" }),
        PropertyMappings = Props(addressColumns),
        ReadProperties = addressColumns,
        Schema = addressSchema,
    });
    var dictionaryEmails = new ReadonlyRepository<Email>(connection, new()
    {
        Entity = "Email",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Email"] = "emails" }),
        PropertyMappings = Props(emailColumns),
        ReadProperties = emailColumns,
        Schema = emailSchema,
    });
    var dictionaryContacts = new ReadonlyRepository<Contact>(connection, new()
    {
        Entity = "Contact",
        EntityMapper = new DictionaryMapper<string>(new Dictionary<string, string> { ["Contact"] = "contacts" }),
        PropertyMappings = Props(contactColumns),
        ReadProperties = contactColumns,
        Schema = contactSchema,
    });
    dictionaryContacts.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), dictionaryAddresses);
    dictionaryContacts.AddEnriched(new("Emails", "Id", "ContactId", Type: EnrichmentPropertyType.List), dictionaryEmails);

    var reads = new
    {
        @default = await Profile("default", () => Collect(defaultRepo.Read())),
        dictionary = await Profile("dictionary", () => Collect(dictionaryContacts.Read())),
    };

    Console.WriteLine(JsonSerializer.Serialize(new
    {
        sample = "mapping_comparison",
        language = "csharp",
        provider,
        rows,
        chunk = Math.Min(chunk, rows),
        createMs,
        createPerSec = createMs == 0 ? 0 : rows / (createMs / 1000),
        counts,
        reads,
    }));
}
finally
{
    await connection.Close();
}

BulkCRUDRepository<T> Repo<T>(IConnection connection, CRUDRepositoryOptions<T> options, string create, string update, string delete)
    where T : new() =>
    batched
        ? new BulkCRUDRepository<T>(connection, options, new BatchCommandIBulkWriteStrategy<T>())
        : new BulkCRUDRepository<T>(connection, options, new StoredProcedureBulkWriteStrategy<T>(create, update, delete));

async Task Each(List<Contact> items, Func<List<Contact>, Task> write)
{
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

static async Task<List<T>> Collect<T>(IAsyncEnumerable<T> rows)
{
    var list = new List<T>();
    await foreach (var row in rows) list.Add(row);
    return list;
}

async Task<object> Profile(string label, Func<Task<List<Contact>>> read)
{
    await read();
    var times = new List<double>();
    for (var iter = 0; iter < readIters; iter++)
    {
        times.Add(await Time(async () => Check(await read(), label)));
    }
    times.Sort();
    var ms = times.Count % 2 == 1 ? times[times.Count / 2] : (times[times.Count / 2 - 1] + times[times.Count / 2]) / 2;
    return new { ms, perSec = rows / (ms / 1000) };
}

void Check(List<Contact> loaded, string label)
{
    if (loaded.Count != rows) throw new InvalidOperationException($"{label} count {loaded.Count}");
    if (loaded.Any(row => row.Addresses.Count is < 2 or > 10 || row.Emails.Count is < 2 or > 6))
        throw new InvalidOperationException($"{label} related");
}

static List<Contact> SeedContacts(int total)
{
    var state = 1u;
    double Next()
    {
        state = unchecked(1664525u * state + 1013904223u);
        return state / (double)0x100000000L;
    }
    int Related(int max = 10) => 2 + (int)(Next() * (max - 1));
    var contacts = new List<Contact>(total);
    for (var i = 1; i <= total; i++)
    {
        var addressCount = Related();
        var emailCount = Related(6);
        contacts.Add(new Contact
        {
            Id = i,
            Name = $"Contact {i}",
            GivenName = $"Given{i}",
            FamilyName = $"Family{i}",
            GenderTypeId = 1 + i % 3,
            Addresses =
            [
                .. Enumerable.Range(0, addressCount).Select(j => new Address
                {
                    Line1 = $"{i} Line {j + 1}",
                    City = $"City{(i + j) % 50}",
                    CountryCode = "US",
                    IsPrimary = j == 0,
                }),
            ],
            Emails =
            [
                .. Enumerable.Range(0, emailCount).Select(j => new Email
                {
                    Address = $"c{i}.e{j + 1}@example.com",
                    DisplayName = $"Email {j + 1}",
                    EmailTypeId = j + 1,
                    IsPrimary = j == 0,
                }),
            ],
        });
    }
    return contacts;
}
