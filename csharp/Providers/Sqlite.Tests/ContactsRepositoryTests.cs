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
            Driver = new SqliteDriver()
        });
        await conn.Open();
        await ContactsRepositoryCase.MatchesEnrichedToFlatMapper(conn, new SqliteCommand(conn), "sqlite");
    }
}
