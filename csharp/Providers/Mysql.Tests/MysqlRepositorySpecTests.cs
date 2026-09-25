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
        _ = MysqlRowTypeConverters.All;
        foreach (var path in RepositoryTestsSpec.Files())
        {
            var isolated = await Isolate(url);
            try
            {
                await RepositoryTestsSpec.Run(path, () =>
                {
                    var conn = new MysqlConnection(new MysqlConnectionOptions
                    {
                        Url = isolated.Url,
                    });
                    return new RepositoryTestsSpec.Session(conn, new MysqlCommand(conn), MysqlRowTypeConverters.All);
                }, CreateTable, Quote);
            }
            finally
            {
                await isolated.Cleanup();
            }
        }
    }

    static string CreateTable(string table, IReadOnlyList<SpecColumn> columns) =>
        new CreateTableStatement(
            Quote(table),
            [.. columns.Select(column => new Column(
                Quote(column.Name),
                column.Id ? $"{Map(column.Type)} PRIMARY KEY AUTO_INCREMENT" : Map(column.Type)))]).GetText();

    static string Quote(string id) => $"`{id.Replace("`", "``")}`";

    static string Map(string canonical) => canonical switch
    {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new InvalidOperationException($"Unknown mysql type \"{canonical}\"")
    };

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
