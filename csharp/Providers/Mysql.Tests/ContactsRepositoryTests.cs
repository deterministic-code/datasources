using Datasources;
using Datasources.CodeGenerators;
using Datasources.Mysql;
using Datasources.SqlSpecs;
using Datasources.Tests.Contact;
using MySqlConnector;
using static Datasources.Cmp;

namespace Datasources.Mysql.Tests;

public class ContactsRepositoryTests
{
    [Fact]
    public async Task MatchesEnrichedContactsServiceToFlatMapper()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions { Url = isolated.Url });
            await conn.Open();
            var command = new MysqlCommand(conn);
            await ContactSchema.Apply(conn, "mysql", new MysqlDriver());
            foreach (var sql in StatementsOf(File.ReadAllText(SeedPath()))
                .Concat(SeedContacts.Sql())
                .Concat(SeedContacts.IdentitySql("mysql")))
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
            await isolated.Cleanup();
        }
    }

    static string SeedPath() => Path.Combine(SqlSpec.SpecsDir(), "contacts", "mysql.sql");

    static IEnumerable<string> StatementsOf(string sql) =>
        sql.Split(';').Select(part => part.Trim()).Where(part => part.Length > 0);

    static async Task<(string Url, Func<Task> Cleanup)> Isolate(string baseUrl)
    {
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE DATABASE `{name}`";
            await create.ExecuteNonQueryAsync();
        }
        var isolated = ReplaceDatabase(baseUrl, name);
        return (isolated, async () =>
        {
            await using var drop = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP DATABASE `{name}`";
            await cmd.ExecuteNonQueryAsync();
        });
    }

    static string ReplaceDatabase(string url, string name)
    {
        if (!url.StartsWith("mysql://", StringComparison.OrdinalIgnoreCase))
            return $"{url};Database={name}";
        var uri = new Uri(url);
        var builder = new UriBuilder(uri) { Path = name };
        return builder.Uri.ToString();
    }
}
