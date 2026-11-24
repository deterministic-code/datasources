using Backend.Types.View;
using Datasources;

await using var ds = Datasource.Default;
var contacts = ds.CreateRepository<Contact>("Contacts");
var ada = await contacts.Create(new Contact
{
    FirstName = "Ada",
    LastName = "Lovelace",
    Notes = "poet",
    Email = "ada@example.com",
    ContactSourceName = "Manual",
    Uuid = Guid.Parse("00000000-0000-0000-0000-000000000001"),
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
    Prefs = new Dictionary<string, LocalePreference>
    {
        ["home"] = new() { Locale = "en-GB", Timezone = "Europe/London" },
    },
});
if (ada.FirstName != "Ada" || ada.Addresses.Count != 2 || ada.Phones.Count != 1
    || ada.Addresses[0].Line2 != "Room 12" || ada.Addresses[1].AddressTypeName != "Work")
    throw new InvalidOperationException("create ada");
var loaded = await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id)));
if (loaded is null || loaded.Addresses[0].Latitude is not null)
    throw new InvalidOperationException("read ada");
RequireAdaChildren(loaded, "Home", "London", "555-0100");
if (loaded.Addresses.First(row => row.Line1 == "1 Strand").AddressTypeName != "Work")
    throw new InvalidOperationException("read work address");

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
if (after is null || after.FirstName != "Ada Byron"
    || after.Addresses.First(row => row.Line1 == "12 Analytical Engine").Country != "FR")
    throw new InvalidOperationException("update ada");
RequireAdaChildren(after, "Work", "Cambridge", "555-0111");
if (after.Addresses.First(row => row.Line1 == "1 College").AddressTypeName != "Home")
    throw new InvalidOperationException("update home address");

var omitted = await contacts.Create(new Contact
{
    FirstName = "Outlier",
    LastName = "Types",
    ContactSourceName = "Manual",
    Uuid = Guid.Parse("00000000-0000-0000-0000-000000000003"),
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
            Score = 10.5m,
            ExternalId = Guid.Parse("550e8400-e29b-41d4-a716-446655440000"),
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
    || valued.Addresses[0].Floor != 2 || valued.Addresses[0].Score != 10.5m
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
            Score = 0m,
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

var connection = await ds.CreateConnection();
if (!connection.IsOpen) await connection.Open();
var addressTypes = new AddressTypesRepository(connection);
var phoneTypes = new PhoneTypesRepository(connection);
var contactSources = new ContactSourcesRepository(connection);
var addressesBulk = new BulkCRUDRepository<Address>(connection, AddressesRepository.Options(connection), new BatchCommandIBulkWriteStrategy<Address>());
addressesBulk.AddEnriched(new("AddressTypeName", "AddressTypeId", "Id", "Name"), addressTypes);
addressesBulk.AddEnriched(new("Description", "AddressTypeId", "Id", "Description"), addressTypes);
var phonesBulk = new BulkCRUDRepository<Phone>(connection, PhonesRepository.Options(connection), new BatchCommandIBulkWriteStrategy<Phone>());
phonesBulk.AddEnriched(new("PhoneTypeName", "PhoneTypeId", "Id", "Name"), phoneTypes);
phonesBulk.AddEnriched(new("Description", "PhoneTypeId", "Id", "Description"), phoneTypes);
var settingsBulk = new BulkCRUDRepository<Backend.Types.Datasource.ContactSetting>(connection, ContactSettingsRepository.Options(connection), new BatchCommandIBulkWriteStrategy<Backend.Types.Datasource.ContactSetting>());
var prefsBulk = new BulkCRUDRepository<Backend.Types.Datasource.ContactPreference>(connection, ContactPreferencesRepository.Options(connection), new BatchCommandIBulkWriteStrategy<Backend.Types.Datasource.ContactPreference>());
var contactsBulk = new BulkCRUDRepository<Contact>(connection, ContactsRepository.Options(connection), new BatchCommandIBulkWriteStrategy<Contact>());
contactsBulk.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), addressesBulk);
contactsBulk.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), phonesBulk);
contactsBulk.AddEnriched(new("Settings", "Id", "ContactId", "Value", EnrichmentPropertyType.Dictionary, DictionaryKey: "Key"), settingsBulk);
contactsBulk.AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Key"), prefsBulk);
contactsBulk.AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), contactSources);
contactsBulk.AddEnriched(new("Description", "ContactSourceId", "Id", "Description"), contactSources);
var bulkItems = Enumerable.Range(0, 10).Select(index =>
{
    var children = 5 + index % 6;
    return new Contact
    {
        FirstName = $"Bulk{index}",
        LastName = "Bulk",
        ContactSourceName = "Manual",
        Addresses = [.. Enumerable.Range(0, children - 1).Select(n => new Address
        {
            AddressTypeName = n % 2 == 0 ? "Home" : "Work",
            Line1 = $"{index}-{n} Bulk Lane",
            City = "London",
            ExternalId = BulkExternalId(index, n),
            VerifiedAt = BulkVerifiedAt(),
        })],
        Phones = [new Phone { PhoneTypeName = "Mobile", Number = $"555-1{index:D3}" }],
    };
}).ToList();
var createdBulk = await contactsBulk.Create(bulkItems);
if (createdBulk.Count != 10 || createdBulk.Any(row => row.Id <= 0)
    || createdBulk.Select(row => row.Id).Distinct().Count() != 10)
    throw new InvalidOperationException("bulk create ids");
for (var index = 0; index < createdBulk.Count; index++)
    RequireBulkContact(createdBulk[index], index);
var loadedAll = new List<Contact>();
await foreach (var row in contacts.Read()) loadedAll.Add(row);
for (var index = 0; index < createdBulk.Count; index++)
{
    var created = createdBulk[index];
    RequireBulkContact(loadedAll.Single(item => item.Id == created.Id), index);
}

var strategy = new LoadLevelOrderReadonlyEnrichmentStrategy<Contact>().WithBatchSize(3);
var loadLevel = new ReadonlyRepository<Contact>(connection, ContactsRepository.Options(connection));
var loadLevelAddresses = new AddressesRepository(connection);
var loadLevelPhones = new PhonesRepository(connection);
var loadLevelSettings = new ContactSettingsRepository(connection);
var loadLevelPrefs = new ContactPreferencesRepository(connection);
var loadLevelSources = new ContactSourcesRepository(connection);
loadLevel.AddEnriched(new("Addresses", "Id", "ContactId", Type: EnrichmentPropertyType.List), loadLevelAddresses, strategy);
loadLevel.AddEnriched(new("Phones", "Id", "ContactId", Type: EnrichmentPropertyType.List), loadLevelPhones, strategy);
loadLevel.AddEnriched(new("Settings", "Id", "ContactId", "Value", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Key"), loadLevelSettings, strategy);
loadLevel.AddEnriched(new("Prefs", "Id", "ContactId", Type: EnrichmentPropertyType.Dictionary, DictionaryKey: "Key"), loadLevelPrefs, strategy);
loadLevel.AddEnriched(new("ContactSourceName", "ContactSourceId", "Id", "Name"), loadLevelSources, strategy);
loadLevel.AddEnriched(new("Description", "ContactSourceId", "Id", "Description"), loadLevelSources, strategy);
var loadLevelAda = await loadLevel.ReadSingle(new Where(new Datasources.Operator.Equals("Id", after.Id)));
if (loadLevelAda is null || loadLevelAda.FirstName != "Ada Byron"
    || loadLevelAda.Settings.GetValueOrDefault("theme") != "sepia"
    || loadLevelAda.Prefs.GetValueOrDefault("home")?.Locale != "en-GB")
    throw new InvalidOperationException("load-level-order ada");
RequireAdaChildren(loadLevelAda, "Work", "Cambridge", "555-0111");
var loadLevelAll = new List<Contact>();
await foreach (var row in loadLevel.Read()) loadLevelAll.Add(row);
for (var index = 0; index < createdBulk.Count; index++)
{
    var created = createdBulk[index];
    var loadLevelRow = loadLevelAll.SingleOrDefault(item => item.Id == created.Id)
        ?? throw new InvalidOperationException("load-level-order bulk");
    RequireBulkContact(loadLevelRow, index);
}

await contacts.Delete(after);
if (await contacts.ReadSingle(new Where(new Datasources.Operator.Equals("Id", ada.Id))) is not null)
    throw new InvalidOperationException("delete ada");

static Guid BulkExternalId(int index, int n) =>
    Guid.Parse($"00000000-0000-0000-0001-{index:D4}{n:D8}");

static DateTime BulkVerifiedAt() =>
    DateTime.Parse("2020-01-02T03:04:05.000Z").ToUniversalTime();

static bool Assigned(DateTime value) => value != default && value.ToUniversalTime().Year >= 2020;

static void RequireBulkContact(Contact row, int index)
{
    if (row.FirstName != $"Bulk{index}" || row.LastName != "Bulk" || row.ContactSourceName != "Manual")
        throw new InvalidOperationException("bulk contact fields");
    if (row.Id <= 0 || row.Uuid == Guid.Empty)
        throw new InvalidOperationException("bulk contact uuid");
    if (!Assigned(row.Created) || !Assigned(row.Updated))
        throw new InvalidOperationException("bulk contact datetime");
    var children = 5 + index % 6;
    if (row.Addresses.Count + row.Phones.Count != children)
        throw new InvalidOperationException("bulk child counts");
    for (var n = 0; n < children - 1; n++)
    {
        var address = row.Addresses.Single(item => item.Line1 == $"{index}-{n} Bulk Lane");
        if (address.AddressTypeName != (n % 2 == 0 ? "Home" : "Work") || address.City != "London")
            throw new InvalidOperationException("bulk address fields");
        if (address.ContactId != row.Id)
            throw new InvalidOperationException("bulk address contact id");
        if (address.Id <= 0 || address.Uuid == Guid.Empty || address.ExternalId != BulkExternalId(index, n))
            throw new InvalidOperationException("bulk address uuid");
        if (!Assigned(address.Created) || !Assigned(address.Updated)
            || address.VerifiedAt?.ToUniversalTime() != BulkVerifiedAt())
            throw new InvalidOperationException("bulk address datetime");
    }
    var phone = row.Phones.Single();
    if (phone.Number != $"555-1{index:D3}" || phone.PhoneTypeName != "Mobile")
        throw new InvalidOperationException("bulk phone fields");
    if (phone.ContactId != row.Id)
        throw new InvalidOperationException("bulk phone contact id");
    if (phone.Id <= 0 || phone.Uuid == Guid.Empty)
        throw new InvalidOperationException("bulk phone uuid");
    if (!Assigned(phone.Created) || !Assigned(phone.Updated))
        throw new InvalidOperationException("bulk phone datetime");
}

static void RequireAdaChildren(Contact row, string engineType, string engineCity, string number)
{
    if (row.Addresses.Count != 2 || row.Phones.Count != 1)
        throw new InvalidOperationException("child counts");
    var engine = row.Addresses.First(address => address.Line1 == "12 Analytical Engine");
    if (engine.AddressTypeName != engineType || engine.City != engineCity)
        throw new InvalidOperationException("engine address");
    if (engine.ContactId != row.Id)
        throw new InvalidOperationException("address contact id");
    if (row.Phones[0].Number != number || row.Phones[0].PhoneTypeName != "Mobile")
        throw new InvalidOperationException("phone");
    if (row.Phones[0].ContactId != row.Id)
        throw new InvalidOperationException("phone contact id");
}
