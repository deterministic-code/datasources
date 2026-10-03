using Datasources;
using Datasources.SqlSpecs;
using Datasources.Sqlite;
using Datasources.Tests.Contact;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

public class ContactsRepositoryTests
{
    [Fact]
    public async Task MatchesEnrichedContactsServiceToFlatMapper()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions
        {
            Filename = ":memory:",
        });
        await conn.Open();
        var command = new SqliteCommand(conn);
        await SchemaSpec.Apply(conn, "contacts");
        foreach (var sql in StatementsOf(File.ReadAllText(SeedPath())).Concat(SeedContacts.Sql()))
            await command.Execute(new Native(sql));

        var (eager, enriched) = await ContactsParity.Run(conn);
        ExpectContactsView.AssertSeed(eager, SeedContacts.Count);
        ExpectContactsView.AssertSeed(enriched, SeedContacts.Count);
        var mapped = ExpectContactsView.With(eager, 2, 1);
        Assert.False(string.IsNullOrEmpty(mapped.contact_source_name));
        Assert.False(string.IsNullOrEmpty(mapped.contact_type_name));
        Assert.Equal(2, mapped.addresses.Length);
        Assert.Equal(eager, enriched);

        var service = ContactsServiceOf.Create(conn);
        var created = await service.Create(ExtraContact.New(await ContactLookupIds.Resolve(conn)));
        Assert.True(created.Id > 0);
        var extra = await service.ReadSingle(new Where("Id", eq, created.Id));
        Assert.NotNull(extra);
        Assert.Equal("Extra", extra.FirstName);
        Assert.Single(extra.Addresses);
        Assert.Equal("9 Extra", extra.Addresses[0].Line1);
        Assert.Equal(extra.Id, extra.Addresses[0].Contact.Id);
        Assert.Equal("Home", extra.Addresses[0].AddressType.Name);
        Assert.Equal("555-extra", Assert.Single(extra.Phones).Number);
        Assert.Equal(extra.Id, extra.Phones[0].Contact.Id);
        Assert.Equal("Mobile", extra.Phones[0].PhoneType.Name);
    }

    [Fact]
    public async Task MapsJoinRowsToNestedContacts()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions
        {
            Filename = ":memory:",
        });
        await conn.Open();
        var command = new SqliteCommand(conn);
        await SchemaSpec.Apply(conn, "contacts");
        foreach (var sql in StatementsOf(File.ReadAllText(SeedPath())).Concat(SeedContacts.Sql()))
            await command.Execute(new Native(sql));

        var rows = new List<Row>();
        await foreach (var row in new CRUDRepository<Row>(conn, new CRUDRepositoryOptions<Row>
        {
            EntityMapper = ContactColumns.JoinEntity,
            PropertyMappings = ContactColumns.Columns,
        }).Read(ContactsParity.JoinSelect()))
            rows.Add(row);
        var contacts = AsContactsView.FromMapped(new FlatToNestedMapper(ContactsView.TypeMap).Map(rows));
        var mapped = ExpectContactsView.With(contacts, 2, 1);
        Assert.Equal(2, mapped.addresses.Length);
        Assert.False(string.IsNullOrEmpty(mapped.contact_source_name));
    }

    [Fact]
    public async Task WritesEnrichedContact()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions
        {
            Filename = ":memory:",
        });
        await conn.Open();
        var command = new SqliteCommand(conn);
        await SchemaSpec.Apply(conn, "contacts");
        foreach (var sql in StatementsOf(File.ReadAllText(SeedPath())))
            await command.Execute(new Native(sql));

        var contactsRepo = new CRUDRepository<Contact>(conn, new CRUDRepositoryOptions<Contact>
        {
            EntityMapper = ContactColumns.ContactEntity,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var typesRepo = new CRUDRepository<ContactType>(conn, new CRUDRepositoryOptions<ContactType>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var sourcesRepo = new CRUDRepository<ContactSource>(conn, new CRUDRepositoryOptions<ContactSource>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var addressTypesRepo = new CRUDRepository<AddressType>(conn, new CRUDRepositoryOptions<AddressType>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var phoneTypesRepo = new CRUDRepository<PhoneType>(conn, new CRUDRepositoryOptions<PhoneType>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var addressesRepo = new CRUDRepository<Address>(conn, new CRUDRepositoryOptions<Address>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        var phonesRepo = new CRUDRepository<Phone>(conn, new CRUDRepositoryOptions<Phone>
        {
            EntityMapper = ContactColumns.Snake,
            PropertyMappings = ContactColumns.Columns,
            Ids = ["Id"],
        });
        addressesRepo.AddEnriched(new("AddressType", "AddressType", "Id"), addressTypesRepo);
        phonesRepo.AddEnriched(new("PhoneType", "PhoneType", "Id"), phoneTypesRepo);
        contactsRepo.AddEnriched(new("ContactType", "ContactType", "Id"), typesRepo);
        contactsRepo.AddEnriched(new("ContactSource", "ContactSource", "Id"), sourcesRepo);
        contactsRepo.AddEnriched(new("Addresses", "Id", "Contact", Type: EnrichmentPropertyType.List), addressesRepo);
        contactsRepo.AddEnriched(new("Phones", "Id", "Contact", Type: EnrichmentPropertyType.List), phonesRepo);

        var created = await contactsRepo.Create(ExtraContact.New(await ContactLookupIds.Resolve(conn)));
        var loaded = await contactsRepo.ReadSingle(new Where("Id", eq, created.Id));

        Assert.True(created.Id > 0);
        Assert.NotNull(loaded);
        Assert.Equal("Extra", loaded.FirstName);
        Assert.Equal("Person", loaded.ContactType.Name);
        Assert.Equal("Web", loaded.ContactSource.Name);
        Assert.Equal("9 Extra", Assert.Single(loaded.Addresses).Line1);
        Assert.Equal("Home", loaded.Addresses[0].AddressType.Name);
        Assert.Equal(loaded.Id, loaded.Addresses[0].Contact.Id);
        Assert.Equal("555-extra", Assert.Single(loaded.Phones).Number);
        Assert.Equal("Mobile", loaded.Phones[0].PhoneType.Name);
        Assert.Equal(loaded.Id, loaded.Phones[0].Contact.Id);
    }

    static string SeedPath() => Path.Combine(SqlSpec.SpecsDir(), "contacts", "sqlite.sql");

    static IEnumerable<string> StatementsOf(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);
}
