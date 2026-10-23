using Datasources;
using Datasources.CodeGenerators;
using Datasources.Postgres;
using Datasources.SqlSpecs;
using Datasources.Tests.Contact;
using Npgsql;
using static Datasources.Cmp;

namespace Datasources.Postgres.Tests;

public class ContactsRepositoryTests
{
    [Fact]
    public async Task MatchesEnrichedContactsServiceToFlatMapper()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE SCHEMA {schema}";
            await create.ExecuteNonQueryAsync();
        }
        try
        {
            var conn = new PostgresConnection(new PostgresConnectionOptions { Url = url, Schema = schema });
            await conn.Open();
            var command = new PostgresCommand(conn);
            await ContactSchema.Apply(conn, "postgres", new PostgresDriver());
            foreach (var sql in StatementsOf(File.ReadAllText(SeedPath()))
                .Concat(SeedContacts.Sql(version: "'\\x'"))
                .Concat(SeedContacts.IdentitySql("postgres")))
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
        finally
        {
            await using var drop = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
            await cmd.ExecuteNonQueryAsync();
        }
    }

    static string SeedPath() => Path.Combine(SqlSpec.SpecsDir(), "contacts", "postgres.sql");

    static IEnumerable<string> StatementsOf(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);
}
