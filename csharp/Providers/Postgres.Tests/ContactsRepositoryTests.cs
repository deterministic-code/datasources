using Datasources.Postgres;
using Datasources.Tests.Contact;
using Npgsql;

namespace Datasources.Postgres.Tests;

public class ContactsRepositoryTests
{
    [Fact]
    public async Task MatchesEnrichedContactsServiceToFlatMapper()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new NpgsqlConnection(PostgresDriver.ToConnectionString(url)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE SCHEMA {schema}";
            await create.ExecuteNonQueryAsync();
        }
        try
        {
            var conn = new PostgresConnection(new PostgresConnectionOptions
            {
                Url = url,
                Schema = schema,
                Driver = new PostgresDriver(schema),
            });
            await conn.Open();
            await ContactsRepositoryCase.MatchesEnrichedToFlatMapper(
                conn, new PostgresCommand(conn), "postgres", @"'\x'");
        }
        finally
        {
            await using var drop = new NpgsqlConnection(PostgresDriver.ToConnectionString(url));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
            await cmd.ExecuteNonQueryAsync();
        }
    }
}
