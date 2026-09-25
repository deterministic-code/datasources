using System.Text.Json;
using Datasources;
using Datasources.Sqlite;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class ApiTests
{
    [Fact]
    public async Task ContactsRoundTripAndLegacyContacts()
    {
        var stamp = new DateTime(2026, 9, 22, 0, 0, 0, DateTimeKind.Utc);
        var now = Clock.UtcNow;
        var uuid = Clock.Uuid;
        var n = 0;
        var lines = new List<string>();
        var record = Environment.GetEnvironmentVariable("RECORD_CONSOLE") == "1";
        ICommandMiddleware middleware = record
            ? new ConsoleCommandMiddleware(line => lines.Add(ExpectConsoleMiddleware.Signature(line)), () => new DateTimeOffset(stamp), new Dictionary<string, string> { ["isVerbose"] = "true" })
            : new ExpectConsoleMiddleware((await File.ReadAllLinesAsync(Log())).Where(line => line.Length > 0).ToArray(), () => new DateTimeOffset(stamp));
        try
        {
        Clock.UtcNow = () => stamp;
        Clock.Uuid = () => Guid.Parse($"00000000-0000-0000-0000-{++n:D12}");
        var api = await Open(middleware);
        var created = await api.Handle("POST", "/contacts/", """
            {
              "first_name": "Ada",
              "last_name": "Lovelace",
              "email": "ada@example.com",
              "contact_source": "Web",
              "contact_type": "Person",
              "addresses": [{ "address_type": "Home", "line1": "12 Analytical Engine", "city": "London" }],
              "phones": [{ "phone_type": "Mobile", "number": "555-0100" }],
              "groups": [{ "id": 1 }]
            }
            """);
        Assert.Equal(201, created.Status);
        Assert.DoesNotContain("m_FirstName", created.Body);
        Assert.DoesNotContain("tbl_Contacts", created.Body);
        Assert.DoesNotContain("contact_type_id", created.Body);
        using var saved = JsonDocument.Parse(created.Body);
        Assert.Equal("Ada", saved.RootElement.GetProperty("first_name").GetString());
        Assert.Equal("Web", saved.RootElement.GetProperty("contact_source").GetString());
        Assert.Equal("Person", saved.RootElement.GetProperty("contact_type").GetString());
        Assert.Equal("Home", saved.RootElement.GetProperty("addresses")[0].GetProperty("address_type").GetString());
        Assert.Equal("Mobile", saved.RootElement.GetProperty("phones")[0].GetProperty("phone_type").GetString());
        Assert.Equal("Friends", saved.RootElement.GetProperty("groups")[0].GetProperty("name").GetString());
        var id = saved.RootElement.GetProperty("id").GetInt32();

        await api.Handle("POST", "/contacts", """
            {
              "first_name": "Grace",
              "last_name": "Hopper",
              "uuid": "22222222-2222-2222-2222-222222222222",
              "created": "2024-01-02T00:00:00Z",
              "contact_source": "Import",
              "contact_type": "Person",
              "addresses": [{ "uuid": "33333333-3333-3333-3333-333333333333", "created": "2024-01-02T00:00:00Z", "address_type": "Home", "line1": "1", "city": "NY" }],
              "phones": [{ "uuid": "44444444-4444-4444-4444-444444444444", "created": "2024-01-02T00:00:00Z", "phone_type": "Home", "number": "1" }],
              "groups": []
            }
            """);
        var filtered = await api.Handle("GET", $"/contacts/?first_name=Ada&last_name=Lovelace&id={id}", null);
        using var list = JsonDocument.Parse(filtered.Body);
        Assert.Equal(1, list.RootElement.GetArrayLength());

        var updated = await api.Handle("PUT", $"/contacts/?id={id}", """
            {
              "first_name": "Ada",
              "last_name": "Byron",
              "email": null,
              "contact_source": "Web",
              "contact_type": "Person",
              "addresses": [{ "address_type": "Work", "line1": "1 Strand", "city": "London" }],
              "phones": [],
              "groups": [{ "id": 1 }, { "id": 2 }]
            }
            """);
        Assert.Equal(200, updated.Status);
        using var changed = JsonDocument.Parse(updated.Body);
        Assert.Equal("Byron", changed.RootElement.GetProperty("last_name").GetString());
        Assert.Equal("Work", changed.RootElement.GetProperty("addresses")[0].GetProperty("address_type").GetString());
        Assert.Equal(0, changed.RootElement.GetProperty("phones").GetArrayLength());
        Assert.Equal(2, changed.RootElement.GetProperty("groups").GetArrayLength());

        Assert.Equal(204, (await api.Handle("DELETE", $"/contacts/?id={id}", null)).Status);
        using var after = JsonDocument.Parse((await api.Handle("get", "/contacts/", null)).Body);
        Assert.Equal(1, after.RootElement.GetArrayLength());

        var legacy = await api.Handle("POST", "/legacy_contacts/", """
            { "key": "ada", "first_name": "Ada", "last_name": "Lovelace", "email": "ada@example.com", "imported_at": "2024-06-01T00:00:00Z" }
            """);
        Assert.Equal(201, legacy.Status);
        using var legacyList = JsonDocument.Parse((await api.Handle("GET", "/legacy_contacts/?first_name=Ada", null)).Body);
        Assert.Equal("ada", legacyList.RootElement[0].GetProperty("key").GetString());
        var legacyPut = await api.Handle("PUT", "/legacy_contacts/?key=ada", """
            { "first_name": "Augusta", "last_name": "Lovelace", "email": "" }
            """);
        Assert.Equal(200, legacyPut.Status);
        Assert.Contains("Augusta", legacyPut.Body);
        Assert.Equal(204, (await api.Handle("DELETE", "/legacy_contacts/?key=ada", null)).Status);
        using var legacyGone = JsonDocument.Parse((await api.Handle("GET", "/legacy_contacts", null)).Body);
        Assert.Equal(0, legacyGone.RootElement.GetArrayLength());
        if (record)
            await File.WriteAllLinesAsync(Log(), lines);
        else
            ((ExpectConsoleMiddleware)middleware).Complete();
        }
        finally
        {
            Clock.UtcNow = now;
            Clock.Uuid = uuid;
        }
    }

    [Fact]
    public async Task RejectsUnknownRoutesLookupsAndMissingRows()
    {
        var api = await Open();
        Assert.Equal(404, (await api.Handle("GET", "/nope", null)).Status);
        Assert.Equal(400, (await api.Handle("POST", "/contacts/", " ")).Status);
        Assert.Equal(400, (await api.Handle("POST", "/contacts/", "null")).Status);
        Assert.Equal(400, (await api.Handle("POST", "/contacts/", "{")).Status);
        var unknown = await api.Handle("POST", "/contacts/", """
            { "first_name": "Ada", "last_name": "Lovelace", "contact_source": "Web", "contact_type": "Nope" }
            """);
        Assert.Equal(400, unknown.Status);
        Assert.Contains("Unknown contact_type", unknown.Body);
        Assert.Equal(400, (await api.Handle("POST", "/contacts/", """
            { "first_name": "Ada", "last_name": "Lovelace", "contact_source": "Web", "contact_type": "Person", "groups": [{}] }
            """)).Status);
        Assert.Equal(400, (await api.Handle("POST", "/contacts/", """
            { "first_name": "Ada", "last_name": "Lovelace", "contact_source": "Web" }
            """)).Status);
        Assert.Equal(400, (await api.Handle("PUT", "/contacts/?id=abc", "{}")).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/contacts/?id=99", """
            { "first_name": "Ada", "last_name": "Lovelace", "contact_source": "Web", "contact_type": "Person" }
            """)).Status);
        Assert.Equal(404, (await api.Handle("DELETE", "/contacts/?id=99", null)).Status);
        Assert.Equal(400, (await api.Handle("DELETE", "/contacts/", null)).Status);
        Assert.Equal(400, (await api.Handle("POST", "/legacy_contacts/", "{}")).Status);
        Assert.Equal(404, (await api.Handle("PUT", "/legacy_contacts/?key=missing", """
            { "first_name": "A", "last_name": "B" }
            """)).Status);
        Assert.Equal(404, (await api.Handle("DELETE", "/legacy_contacts/?key=missing", null)).Status);
        Assert.Equal(400, (await api.Handle("DELETE", "/legacy_contacts/?key=", null)).Status);
        Assert.Equal(200, (await api.Handle("GET", "/contacts/?id=&lonely&first_name=Ada%20Lovelace", null)).Status);
    }

    static async Task<Api> Open(ICommandMiddleware? middleware = null)
    {
        var connection = new SqliteDriver().Connection(new Dictionary<string, string?> { ["filename"] = ":memory:" });
        await connection.Open();
        await Migrate.Apply(connection.CreateCommand());
        var contacts = new ContactsRepository(connection);
        var legacy = new LegacyContactRepository(connection);
        if (middleware is not null)
        {
            contacts.AddMiddleware(middleware);
            legacy.AddMiddleware(middleware);
        }
        return new Api(contacts, legacy);
    }

    static string Log()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var samples = Path.Combine(dir.FullName, "samples");
            if (!Directory.Exists(samples)) continue;
            var path = Path.Combine(samples, "contacts-crud-enriched-medium-rest-shared");
            Directory.CreateDirectory(path);
            return Path.Combine(path, "sqlite.log");
        }
        throw new InvalidOperationException("shared spec");
    }
}
