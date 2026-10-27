using Backend.Types.View;
using Datasources;
using Datasources.Sqlite;

var url = Environment.GetEnvironmentVariable("DATASOURCE_URL")
    ?? throw new InvalidOperationException("DATASOURCE_URL");
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?>
{
    ["DATASOURCE_URL"] = url,
});
await (await ds.CreateConnection()).Open();
var contacts = ds.CreateRepository<Contact>("Contacts");
var created = await contacts.Create(new Contact
{
    FirstName = "Ada",
    LastName = "Lovelace",
    ContactSourceName = "Manual",
    Uuid = "00000000-0000-0000-0000-000000000001",
    Version = [0],
    Addresses = [],
    Phones = [],
    Settings = [],
    Prefs = [],
});
if (created.FirstName != "Ada")
    throw new InvalidOperationException("create");
var loaded = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", created.Id)));
if (loaded?.FirstName != "Ada")
    throw new InvalidOperationException("read");
