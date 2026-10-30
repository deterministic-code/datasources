using Backend.Types.View;
using Datasources;
using Datasources.Sqlite;

var url = Environment.GetEnvironmentVariable("DATASOURCE_URL")
    ?? throw new InvalidOperationException("DATASOURCE_URL");
var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?>
{
    ["DATASOURCE_URL"] = url,
});
var contacts = ds.CreateRepository<Contact>("Contacts");
var ada = await contacts.Create(new Contact
{
    FirstName = "Ada",
    LastName = "Lovelace",
    Notes = "poet",
    Email = "ada@example.com",
    ContactSourceName = "Manual",
    Uuid = "00000000-0000-0000-0000-000000000001",
    Version = [0],
    Addresses =
    [
        new Address
        {
            AddressTypeName = "Home",
            Line1 = "12 Analytical Engine",
            Line2 = "Room 12",
            City = "London",
            Region = "England",
            PostalCode = "SW1A 1AA",
            Country = "GB",
        },
        new Address
        {
            AddressTypeName = "Work",
            Line1 = "1 Strand",
            City = "London",
        },
    ],
    Phones = [new Phone { PhoneTypeName = "Mobile", Number = "555-0100" }],
    Settings = new Dictionary<string, string> { ["theme"] = "dark" },
    Prefs = new Dictionary<string, LocalePreferences>
    {
        ["home"] = new() { Locale = "en-GB", Timezone = "Europe/London" },
    },
});
if (ada.FirstName != "Ada" || ada.Addresses.Count != 2 || ada.Phones.Count != 1
    || ada.Addresses[0].Line2 != "Room 12" || ada.Addresses[1].AddressTypeName != "Work")
    throw new InvalidOperationException("create ada");
var loaded = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id)));
if (loaded?.Addresses.Count != 2 || loaded.Addresses[0].Latitude is not null)
    throw new InvalidOperationException("read ada");

await contacts.Update(new Contact
{
    Id = ada.Id,
    FirstName = "Ada Byron",
    LastName = ada.LastName,
    ContactSourceName = "Manual",
    Uuid = ada.Uuid,
    Version = ada.Version,
    Addresses =
    [
        new Address
        {
            Id = ada.Addresses[0].Id,
            AddressTypeName = "Work",
            Line1 = ada.Addresses[0].Line1,
            Line2 = "",
            City = "Cambridge",
            Region = "",
            Country = "FR",
        },
        new Address { AddressTypeName = "Home", Line1 = "1 College", City = "Cambridge", Country = "GB" },
    ],
    Phones = [new Phone { Id = ada.Phones[0].Id, PhoneTypeName = "Mobile", Number = "555-0111" }],
    Settings = new Dictionary<string, string> { ["theme"] = "sepia" },
    Prefs = ada.Prefs,
});
var after = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id)));
if (after?.FirstName != "Ada Byron" || after.Addresses.Count != 2
    || after.Addresses.First(row => row.Line1 == "12 Analytical Engine").AddressTypeName != "Work"
    || after.Addresses.First(row => row.Line1 == "12 Analytical Engine").Country != "FR")
    throw new InvalidOperationException("update ada");

var omitted = await contacts.Create(new Contact
{
    FirstName = "Outlier",
    LastName = "Types",
    ContactSourceName = "Manual",
    Uuid = "00000000-0000-0000-0000-000000000003",
    Version = [0],
    Addresses = [new Address { AddressTypeName = "Home", Line1 = "1 Null Lane", City = "London" }],
    Phones = [],
    Settings = [],
    Prefs = [],
});
if (omitted.Addresses[0].Latitude is not null || omitted.Addresses[0].IsVerified is not null)
    throw new InvalidOperationException("outlier omit");
await contacts.Update(new Contact
{
    Id = omitted.Id,
    FirstName = omitted.FirstName,
    LastName = omitted.LastName,
    ContactSourceName = "Manual",
    Uuid = omitted.Uuid,
    Version = omitted.Version,
    Addresses =
    [
        new Address
        {
            Id = omitted.Addresses[0].Id,
            AddressTypeName = "Home",
            Line1 = "1 Null Lane",
            City = "London",
            Latitude = 51,
            LabelNote = "side",
            Floor = 2,
            Score = "10.5",
            ExternalId = "550e8400-e29b-41d4-a716-446655440000",
            IsVerified = true,
            VerifiedAt = DateTime.Parse("2020-01-02T03:04:05.000Z").ToUniversalTime(),
            Stamp = [1],
        },
    ],
    Phones = [],
    Settings = [],
    Prefs = [],
});
var valued = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", omitted.Id)));
if (valued?.Addresses[0].Latitude != 51 || valued.Addresses[0].LabelNote != "side"
    || valued.Addresses[0].Floor != 2 || valued.Addresses[0].Score != "10.5"
    || valued.Addresses[0].IsVerified != true)
    throw new InvalidOperationException("outlier value");
await contacts.Update(new Contact
{
    Id = omitted.Id,
    FirstName = omitted.FirstName,
    LastName = omitted.LastName,
    ContactSourceName = "Manual",
    Uuid = omitted.Uuid,
    Version = valued.Version,
    Addresses =
    [
        new Address
        {
            Id = omitted.Addresses[0].Id,
            AddressTypeName = "Home",
            Line1 = "1 Null Lane",
            City = "London",
            Latitude = 0,
            LabelNote = "",
            Floor = 0,
            Score = "0",
            IsVerified = false,
        },
    ],
    Phones = [],
    Settings = [],
    Prefs = [],
});
var zeroed = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", omitted.Id)));
if (zeroed?.Addresses[0].Latitude != 0 || zeroed.Addresses[0].LabelNote != ""
    || zeroed.Addresses[0].Floor != 0 || zeroed.Addresses[0].IsVerified != false)
    throw new InvalidOperationException("outlier zero");

await contacts.Delete(after);
if (await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id))) is not null)
    throw new InvalidOperationException("delete ada");
