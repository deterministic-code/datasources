using Datasources;
using Datasources.Mysql;
using Datasources.SqlSpecs;
using MySqlConnector;

namespace Datasources.Mysql.Tests;

public class MysqlRepositorySpecTests
{
    [Fact]
    public async Task RepositorySpecs()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        _ = MysqlTypes.All;
        foreach (var path in RepositorySpec.Files())
        {
            var isolated = await Isolate(url);
            try
            {
                await RepositorySpec.Run(path, () =>
                {
                    var conn = new MysqlConnection(new MysqlConnectionOptions
                    {
                        Url = isolated.Url,
                        Driver = new MysqlDriver(),
                    });
                    return new RepositorySpec.Session(conn, new MysqlCommand(conn), MysqlTypes.All);
                }, CreateTable, MysqlQuote.Quote);
            }
            finally
            {
                await isolated.Cleanup();
            }
        }
    }

    static string CreateTable(string table, IReadOnlyList<SpecColumn> columns) =>
        new CreateTableStatement(
            MysqlQuote.Quote(table),
            [.. columns.Select(column => new Column(
                MysqlQuote.Quote(column.Name),
                column.Id ? $"{MysqlColumnType.Map(column.Type)} PRIMARY KEY AUTO_INCREMENT" : MysqlColumnType.Map(column.Type)))]).GetText();

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
