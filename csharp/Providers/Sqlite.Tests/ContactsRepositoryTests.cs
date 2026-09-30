using Datasources;
using Datasources.SqlSpecs;
using Datasources.Sqlite;
using Datasources.Tests.Contact;

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
        Assert.True(created.id > 0);
        var extra = await service.ReadSingle(new Where(new Operator.Equals("id", Value.From(created.id))));
        Assert.NotNull(extra);
        Assert.Equal("Extra", extra.first_name);
        Assert.Single(extra.addresses);
        Assert.Equal("9 Extra", extra.addresses[0].line1);
        Assert.Equal(extra.id, extra.addresses[0].contact_id);
        Assert.Equal("Home", extra.addresses[0].address_type_name);
        Assert.Equal("555-extra", Assert.Single(extra.phones).number);
        Assert.Equal(extra.id, extra.phones[0].contact_id);
        Assert.Equal("Mobile", extra.phones[0].phone_type_name);
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
            Entity = "contacts_base"
        }).Read(ContactsJoinQuery.Create()))
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
            Entity = "contacts_base",
            ReadProperties = [
                "id", "uuid", "created", "updated", "version",
                "contact_source_id", "contact_type_id", "first_name", "last_name", "email", "notes"
            ],
        });
        var typesRepo = new CRUDRepository<ContactType>(conn, new CRUDRepositoryOptions<ContactType>
        {
            Entity = "contact_type",
        });
        var sourcesRepo = new CRUDRepository<ContactSource>(conn, new CRUDRepositoryOptions<ContactSource>
        {
            Entity = "contact_source",
        });
        var addressTypesRepo = new CRUDRepository<AddressType>(conn, new CRUDRepositoryOptions<AddressType>
        {
            Entity = "address_type",
        });
        var phoneTypesRepo = new CRUDRepository<PhoneType>(conn, new CRUDRepositoryOptions<PhoneType>
        {
            Entity = "phone_type",
        });
        var addressesRepo = new CRUDRepository<Address>(conn, new CRUDRepositoryOptions<Address>
        {
            Entity = "address",
            ReadProperties = [
                "id", "uuid", "created", "updated", "version", "contact_id", "address_type_id",
                "line1", "line2", "city", "region", "postal_code", "country"
            ],
        });
        var phonesRepo = new CRUDRepository<Phone>(conn, new CRUDRepositoryOptions<Phone>
        {
            Entity = "phone",
            ReadProperties = [
                "id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number"
            ],
        });
        addressesRepo.AddEnriched(new("address_type_name", "address_type_id", "id", "name"), addressTypesRepo);
        phonesRepo.AddEnriched(new("phone_type_name", "phone_type_id", "id", "name"), phoneTypesRepo);
        contactsRepo.AddEnriched(new("contact_type_name", "contact_type_id", "id", "name"), typesRepo);
        contactsRepo.AddEnriched(new("contact_source_name", "contact_source_id", "id", "name"), sourcesRepo);
        contactsRepo.AddEnriched(new("addresses", "id", "contact_id", Many: true), addressesRepo);
        contactsRepo.AddEnriched(new("phones", "id", "contact_id", Many: true), phonesRepo);

        var created = await contactsRepo.Create(ExtraContact.New(await ContactLookupIds.Resolve(conn)));
        var loaded = await contactsRepo.ReadSingle(new Where(new Operator.Equals("id", Value.From(created.id))));

        Assert.True(created.id > 0);
        Assert.NotNull(loaded);
        Assert.Equal("Extra", loaded.first_name);
        Assert.Equal("Person", loaded.contact_type_name);
        Assert.Equal("Web", loaded.contact_source_name);
        Assert.Equal("9 Extra", Assert.Single(loaded.addresses).line1);
        Assert.Equal("Home", loaded.addresses[0].address_type_name);
        Assert.Equal(loaded.id, loaded.addresses[0].contact_id);
        Assert.Equal("555-extra", Assert.Single(loaded.phones).number);
        Assert.Equal("Mobile", loaded.phones[0].phone_type_name);
        Assert.Equal(loaded.id, loaded.phones[0].contact_id);
    }

    static string SeedPath() => Path.Combine(SqlSpec.SpecsDir(), "contacts", "sqlite.sql");

    static IEnumerable<string> StatementsOf(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);
}
