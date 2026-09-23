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
        foreach (var sql in StatementsOf(File.ReadAllText(SchemaPath()))
            .Concat(SeedContacts.Sql()))
            await command.Execute(sql);

        var (eager, enriched) = await ContactsParity.Run(conn);
        ExpectContactsView.AssertSeed(eager, SeedContacts.Count);
        ExpectContactsView.AssertSeed(enriched, SeedContacts.Count);
        var mapped = ExpectContactsView.With(eager, 2, 1);
        Assert.False(string.IsNullOrEmpty(mapped.contact_source_name));
        Assert.False(string.IsNullOrEmpty(mapped.contact_type_name));
        Assert.Equal(2, mapped.addresses.Length);
        Assert.Equal(eager, enriched);

        var service = ContactsServiceOf.Create(conn);
        var created = ExtraContact.New(await ContactLookupIds.Resolve(conn));
        await service.Create(created);
        var extra = await service.ReadSingle(new WhereClause(new Operator.Equals("id", Value.From(created.id))));
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
        foreach (var sql in StatementsOf(File.ReadAllText(SchemaPath())).Concat(SeedContacts.Sql()))
            await command.Execute(sql);

        var rows = new List<Row>();
        await foreach (var row in new CRUDRepository<Row>(new RepositoryBuilderOptions<Row>
        {
            Connection = conn,
            Entity = "contacts_base"
        }).Read(ContactsJoinQuery.Create()))
            rows.Add(row);
        var contacts = AsContactsView.FromMapped(new FlatToNestedMapper(ContactsView.TypeMap).Map(rows));
        var mapped = ExpectContactsView.With(contacts, 2, 1);
        Assert.Equal(2, mapped.addresses.Length);
        Assert.False(string.IsNullOrEmpty(mapped.contact_source_name));
    }

    static string SchemaPath()
    {
        var root = Directory.GetParent(SqlSpec.SpecsDir())?.FullName
            ?? throw new DirectoryNotFoundException("repo root");
        return Path.Combine(root, "csharp", "Datasources.Tests", "Contact", "sqlite.sql");
    }

    static IEnumerable<string> StatementsOf(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);
}
