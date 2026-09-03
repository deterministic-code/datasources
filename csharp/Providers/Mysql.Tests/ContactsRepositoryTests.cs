using Datasources.Mysql;
using Datasources.Tests.Contact;
using MySqlConnector;

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
            var conn = new MysqlConnection(new MysqlConnectionOptions
            {
                Url = isolated.Url,
                Driver = new MysqlDriver(),
            });
            await conn.Open();
            await ContactsRepositoryCase.MatchesEnrichedToFlatMapper(conn, new MysqlCommand(conn), "mysql");
        }
        finally
        {
            await isolated.Cleanup();
        }
    }

    static async Task<(string Url, Func<Task> Cleanup)> Isolate(string baseUrl)
    {
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new MySqlConnection(MysqlDriver.ToConnectionString(baseUrl)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE DATABASE `{name}`";
            await create.ExecuteNonQueryAsync();
        }
        return (ReplaceDatabase(baseUrl, name), async () =>
        {
            await using var drop = new MySqlConnection(MysqlDriver.ToConnectionString(baseUrl));
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
